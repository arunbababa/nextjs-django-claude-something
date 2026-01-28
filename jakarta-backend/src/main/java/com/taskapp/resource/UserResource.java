package com.taskapp.resource;

import com.taskapp.dto.ErrorDTO;
import com.taskapp.dto.UserDTO;
import com.taskapp.entity.User;
import com.taskapp.filter.Authenticated;
import jakarta.ws.rs.*;
import jakarta.ws.rs.container.ContainerRequestContext;
import jakarta.ws.rs.core.Context;
import jakarta.ws.rs.core.MediaType;
import jakarta.ws.rs.core.Response;

@Path("/users")
@Produces(MediaType.APPLICATION_JSON)
@Consumes(MediaType.APPLICATION_JSON)
public class UserResource {

    @GET
    @Path("/me")
    @Authenticated
    public Response getCurrentUser(@Context ContainerRequestContext requestContext) {
        User user = (User) requestContext.getProperty("currentUser");
        if (user == null) {
            return Response.status(Response.Status.UNAUTHORIZED)
                    .entity(new ErrorDTO("detail", "Authentication credentials were not provided."))
                    .build();
        }

        return Response.ok(UserDTO.fromEntity(user)).build();
    }
}
