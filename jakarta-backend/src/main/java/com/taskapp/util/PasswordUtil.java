package com.taskapp.util;

import jakarta.enterprise.context.ApplicationScoped;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.security.SecureRandom;
import java.util.Base64;

@ApplicationScoped
public class PasswordUtil {

    private static final int SALT_LENGTH = 16;
    private static final String ALGORITHM = "SHA-256";
    private static final int ITERATIONS = 100000;

    public String hashPassword(String password) {
        try {
            // Generate salt
            SecureRandom random = new SecureRandom();
            byte[] salt = new byte[SALT_LENGTH];
            random.nextBytes(salt);

            // Hash password with salt
            byte[] hash = pbkdf2(password, salt, ITERATIONS);

            // Combine salt and hash
            byte[] combined = new byte[salt.length + hash.length];
            System.arraycopy(salt, 0, combined, 0, salt.length);
            System.arraycopy(hash, 0, combined, salt.length, hash.length);

            return Base64.getEncoder().encodeToString(combined);
        } catch (NoSuchAlgorithmException e) {
            throw new RuntimeException("Failed to hash password", e);
        }
    }

    public boolean verifyPassword(String password, String storedHash) {
        try {
            byte[] combined = Base64.getDecoder().decode(storedHash);

            // Extract salt
            byte[] salt = new byte[SALT_LENGTH];
            System.arraycopy(combined, 0, salt, 0, SALT_LENGTH);

            // Extract stored hash
            byte[] storedHashBytes = new byte[combined.length - SALT_LENGTH];
            System.arraycopy(combined, SALT_LENGTH, storedHashBytes, 0, storedHashBytes.length);

            // Compute hash with extracted salt
            byte[] computedHash = pbkdf2(password, salt, ITERATIONS);

            // Compare hashes
            return MessageDigest.isEqual(storedHashBytes, computedHash);
        } catch (Exception e) {
            return false;
        }
    }

    private byte[] pbkdf2(String password, byte[] salt, int iterations) throws NoSuchAlgorithmException {
        MessageDigest digest = MessageDigest.getInstance(ALGORITHM);
        byte[] passwordBytes = password.getBytes(StandardCharsets.UTF_8);

        byte[] result = new byte[digest.getDigestLength()];
        byte[] block = new byte[salt.length + 4];
        System.arraycopy(salt, 0, block, 0, salt.length);
        block[salt.length] = 0;
        block[salt.length + 1] = 0;
        block[salt.length + 2] = 0;
        block[salt.length + 3] = 1;

        digest.update(passwordBytes);
        byte[] u = digest.digest(block);
        System.arraycopy(u, 0, result, 0, u.length);

        for (int i = 1; i < iterations; i++) {
            digest.reset();
            digest.update(passwordBytes);
            u = digest.digest(u);
            for (int j = 0; j < result.length; j++) {
                result[j] ^= u[j];
            }
        }

        return result;
    }
}
