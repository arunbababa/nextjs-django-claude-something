package com.taskapp.resource;

import com.taskapp.dto.AuthDTO;
import com.taskapp.dto.ErrorDTO;
import com.taskapp.entity.User;
import com.taskapp.filter.Authenticated;
import com.taskapp.service.AuthService;
import jakarta.inject.Inject;
import jakarta.validation.Valid;
import jakarta.ws.rs.*;
import jakarta.ws.rs.container.ContainerRequestContext;
import jakarta.ws.rs.core.Context;
import jakarta.ws.rs.core.MediaType;
import jakarta.ws.rs.core.Response;

@Path("/auth")
@Produces(MediaType.APPLICATION_JSON)
@Consumes(MediaType.APPLICATION_JSON)
public class AuthResource {

    @Inject
    private AuthService authService;

    @POST
    @Path("/register")
    public Response register(@Valid AuthDTO.RegisterRequest request) {
        try {
            AuthDTO.AuthResponse authResponse = authService.register(request);
            return Response.status(Response.Status.CREATED)
                    .entity(authResponse)
                    .build();
        } catch (AuthService.AuthException e) {
            ErrorDTO error = new ErrorDTO(e.getField(), e.getMessage());
            return Response.status(Response.Status.BAD_REQUEST)
                    .entity(error)
                    .build();
        }
    }

    @POST
    @Path("/login")
    public Response login(@Valid AuthDTO.LoginRequest request) {
        try {
            AuthDTO.AuthResponse authResponse = authService.login(request);
            return Response.ok(authResponse).build();
        } catch (AuthService.AuthException e) {
            ErrorDTO error = new ErrorDTO("detail", e.getMessage());
            return Response.status(Response.Status.BAD_REQUEST)
                    .entity(error)
                    .build();
        }
    }

    @POST
    @Path("/logout")
    @Authenticated
    public Response logout(@Context ContainerRequestContext requestContext) {
        User user = (User) requestContext.getProperty("currentUser");
        if (user == null) {
            return Response.status(Response.Status.UNAUTHORIZED)
                    .entity(new ErrorDTO("detail", "Authentication credentials were not provided."))
                    .build();
        }

        authService.logout(user);
        return Response.ok().entity("{\"detail\": \"Successfully logged out.\"}").build();
    }
}
