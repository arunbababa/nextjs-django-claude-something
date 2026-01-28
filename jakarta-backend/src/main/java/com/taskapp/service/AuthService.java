package com.taskapp.service;

import com.taskapp.dto.AuthDTO;
import com.taskapp.dto.UserDTO;
import com.taskapp.entity.Token;
import com.taskapp.entity.User;
import com.taskapp.repository.TokenRepository;
import com.taskapp.repository.UserRepository;
import com.taskapp.util.PasswordUtil;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;
import jakarta.transaction.Transactional;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import java.util.regex.Pattern;

@ApplicationScoped
public class AuthService {

    private static final Pattern USERNAME_PATTERN = Pattern.compile("^[\\w.@+-]+$");
    private static final List<String> COMMON_PASSWORDS = List.of(
            "password", "12345678", "123456789", "qwerty", "abc123",
            "password1", "password123", "admin", "letmein", "welcome"
    );

    @Inject
    private UserRepository userRepository;

    @Inject
    private TokenRepository tokenRepository;

    @Inject
    private PasswordUtil passwordUtil;

    @Transactional
    public AuthDTO.AuthResponse register(AuthDTO.RegisterRequest request) throws AuthException {
        // Validate passwords match
        if (!request.getPassword().equals(request.getPassword2())) {
            throw new AuthException("password2", "Passwords don't match.");
        }

        // Validate username format
        List<String> usernameErrors = validateUsername(request.getUsername());
        if (!usernameErrors.isEmpty()) {
            throw new AuthException("username", usernameErrors.get(0));
        }

        // Validate password strength
        List<String> passwordErrors = validatePassword(request.getPassword());
        if (!passwordErrors.isEmpty()) {
            throw new AuthException("password", passwordErrors.get(0));
        }

        // Check username uniqueness
        if (userRepository.existsByUsername(request.getUsername())) {
            throw new AuthException("username", "A user with that username already exists.");
        }

        // Check email uniqueness
        if (userRepository.existsByEmail(request.getEmail())) {
            throw new AuthException("email", "A user with that email already exists.");
        }

        // Create user
        User user = new User();
        user.setUsername(request.getUsername());
        user.setEmail(request.getEmail());
        user.setPassword(passwordUtil.hashPassword(request.getPassword()));

        user = userRepository.save(user);

        // Generate token
        Token token = tokenRepository.getOrCreate(user);

        return new AuthDTO.AuthResponse(token.getKey(), UserDTO.fromEntity(user));
    }

    @Transactional
    public AuthDTO.AuthResponse login(AuthDTO.LoginRequest request) throws AuthException {
        Optional<User> userOpt = userRepository.findByUsername(request.getUsername());

        if (userOpt.isEmpty()) {
            throw new AuthException("detail", "Unable to log in with provided credentials.");
        }

        User user = userOpt.get();

        if (!passwordUtil.verifyPassword(request.getPassword(), user.getPassword())) {
            throw new AuthException("detail", "Unable to log in with provided credentials.");
        }

        Token token = tokenRepository.getOrCreate(user);

        return new AuthDTO.AuthResponse(token.getKey(), UserDTO.fromEntity(user));
    }

    @Transactional
    public void logout(User user) {
        tokenRepository.deleteByUserId(user.getId());
    }

    public Optional<User> validateToken(String tokenKey) {
        Optional<Token> tokenOpt = tokenRepository.findByKey(tokenKey);
        return tokenOpt.map(Token::getUser);
    }

    private List<String> validateUsername(String username) {
        List<String> errors = new ArrayList<>();

        if (!USERNAME_PATTERN.matcher(username).matches()) {
            errors.add("Username may only contain letters, numbers, and @/./+/-/_ characters.");
        }

        return errors;
    }

    private List<String> validatePassword(String password) {
        List<String> errors = new ArrayList<>();

        if (password.length() < 8) {
            errors.add("Password must be at least 8 characters long.");
        }

        // Check if all digits
        boolean allDigits = password.chars().allMatch(Character::isDigit);
        if (allDigits) {
            errors.add("Password can't be entirely numeric.");
        }

        // Check common passwords
        String lowerPassword = password.toLowerCase();
        for (String common : COMMON_PASSWORDS) {
            if (lowerPassword.equals(common)) {
                errors.add("Password is too common.");
                break;
            }
        }

        return errors;
    }

    public static class AuthException extends Exception {
        private final String field;

        public AuthException(String field, String message) {
            super(message);
            this.field = field;
        }

        public String getField() {
            return field;
        }
    }
}
