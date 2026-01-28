package com.taskapp.util;

import jakarta.enterprise.context.ApplicationScoped;
import java.security.SecureRandom;

@ApplicationScoped
public class TokenUtil {

    private static final int TOKEN_LENGTH = 20;
    private static final SecureRandom secureRandom = new SecureRandom();

    public String generateToken() {
        byte[] bytes = new byte[TOKEN_LENGTH];
        secureRandom.nextBytes(bytes);
        StringBuilder sb = new StringBuilder();
        for (byte b : bytes) {
            sb.append(String.format("%02x", b));
        }
        return sb.toString();
    }
}
