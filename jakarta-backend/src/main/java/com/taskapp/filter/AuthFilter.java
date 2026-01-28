package com.taskapp.filter;

import com.taskapp.entity.User;
import com.taskapp.service.AuthService;
import jakarta.inject.Inject;
import jakarta.ws.rs.container.ContainerRequestContext;
import jakarta.ws.rs.container.ContainerRequestFilter;
import jakarta.ws.rs.core.Response;
import jakarta.ws.rs.ext.Provider;

import java.io.IOException;
import java.util.Optional;

@Provider
@Authenticated
public class AuthFilter implements ContainerRequestFilter {

    @Inject
    private AuthService authService;

    @Override
    public void filter(ContainerRequestContext requestContext) throws IOException {
        String authHeader = requestContext.getHeaderString("Authorization");

        if (authHeader == null || authHeader.isEmpty()) {
            requestContext.abortWith(Response.status(Response.Status.UNAUTHORIZED)
                    .entity("{\"detail\": \"Authentication credentials were not provided.\"}")
                    .build());
            return;
        }

        // Parse "Token <key>" format
        String[] parts = authHeader.split(" ", 2);
        if (parts.length != 2 || !"Token".equals(parts[0])) {
            requestContext.abortWith(Response.status(Response.Status.UNAUTHORIZED)
                    .entity("{\"detail\": \"Invalid token header. No credentials provided.\"}")
                    .build());
            return;
        }

        String tokenKey = parts[1];

        // Validate token
        Optional<User> userOpt = authService.validateToken(tokenKey);
        if (userOpt.isEmpty()) {
            requestContext.abortWith(Response.status(Response.Status.UNAUTHORIZED)
                    .entity("{\"detail\": \"Invalid token.\"}")
                    .build());
            return;
        }

        // Store user in request context for later use
        requestContext.setProperty("currentUser", userOpt.get());
    }
}
