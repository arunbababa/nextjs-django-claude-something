package com.taskapp.resource;

import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.Produces;
import jakarta.ws.rs.core.MediaType;
import jakarta.ws.rs.core.Response;
import java.util.Map;

@Path("/health")
@Produces(MediaType.APPLICATION_JSON)
public class HealthResource {

    @GET
    public Response healthCheck() {
        return Response.ok(Map.of(
                "status", "healthy",
                "backend", "jakarta-ee"
        )).build();
    }
}
