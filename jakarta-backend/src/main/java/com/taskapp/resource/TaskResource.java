package com.taskapp.resource;

import com.taskapp.dto.ErrorDTO;
import com.taskapp.dto.TaskDTO;
import com.taskapp.entity.User;
import com.taskapp.filter.Authenticated;
import com.taskapp.service.TaskService;
import jakarta.inject.Inject;
import jakarta.validation.Valid;
import jakarta.ws.rs.*;
import jakarta.ws.rs.container.ContainerRequestContext;
import jakarta.ws.rs.core.Context;
import jakarta.ws.rs.core.MediaType;
import jakarta.ws.rs.core.Response;

import java.util.List;
import java.util.Optional;

@Path("/tasks")
@Produces(MediaType.APPLICATION_JSON)
@Consumes(MediaType.APPLICATION_JSON)
@Authenticated
public class TaskResource {

    @Inject
    private TaskService taskService;

    @GET
    @Path("/")
    public Response listTasks(@Context ContainerRequestContext requestContext) {
        User user = (User) requestContext.getProperty("currentUser");
        if (user == null) {
            return Response.status(Response.Status.UNAUTHORIZED)
                    .entity(new ErrorDTO("detail", "Authentication credentials were not provided."))
                    .build();
        }

        List<TaskDTO.TaskResponse> tasks = taskService.getTasksByUser(user);
        return Response.ok(tasks).build();
    }

    @GET
    public Response listTasksNoSlash(@Context ContainerRequestContext requestContext) {
        return listTasks(requestContext);
    }

    @POST
    @Path("/")
    public Response createTask(@Valid TaskDTO.CreateRequest request,
                               @Context ContainerRequestContext requestContext) {
        User user = (User) requestContext.getProperty("currentUser");
        if (user == null) {
            return Response.status(Response.Status.UNAUTHORIZED)
                    .entity(new ErrorDTO("detail", "Authentication credentials were not provided."))
                    .build();
        }

        TaskDTO.TaskResponse task = taskService.createTask(request, user);
        return Response.status(Response.Status.CREATED).entity(task).build();
    }

    @POST
    public Response createTaskNoSlash(@Valid TaskDTO.CreateRequest request,
                                      @Context ContainerRequestContext requestContext) {
        return createTask(request, requestContext);
    }

    @GET
    @Path("/{id}")
    public Response getTask(@PathParam("id") Long id,
                            @Context ContainerRequestContext requestContext) {
        User user = (User) requestContext.getProperty("currentUser");
        if (user == null) {
            return Response.status(Response.Status.UNAUTHORIZED)
                    .entity(new ErrorDTO("detail", "Authentication credentials were not provided."))
                    .build();
        }

        Optional<TaskDTO.TaskResponse> task = taskService.getTaskByIdAndUser(id, user);
        if (task.isEmpty()) {
            return Response.status(Response.Status.NOT_FOUND)
                    .entity(new ErrorDTO("detail", "Task not found."))
                    .build();
        }

        return Response.ok(task.get()).build();
    }

    @GET
    @Path("/{id}/")
    public Response getTaskWithSlash(@PathParam("id") Long id,
                                     @Context ContainerRequestContext requestContext) {
        return getTask(id, requestContext);
    }

    @PATCH
    @Path("/{id}")
    public Response updateTask(@PathParam("id") Long id,
                               @Valid TaskDTO.UpdateRequest request,
                               @Context ContainerRequestContext requestContext) {
        User user = (User) requestContext.getProperty("currentUser");
        if (user == null) {
            return Response.status(Response.Status.UNAUTHORIZED)
                    .entity(new ErrorDTO("detail", "Authentication credentials were not provided."))
                    .build();
        }

        Optional<TaskDTO.TaskResponse> task = taskService.updateTask(id, request, user);
        if (task.isEmpty()) {
            return Response.status(Response.Status.NOT_FOUND)
                    .entity(new ErrorDTO("detail", "Task not found."))
                    .build();
        }

        return Response.ok(task.get()).build();
    }

    @PATCH
    @Path("/{id}/")
    public Response updateTaskWithSlash(@PathParam("id") Long id,
                                        @Valid TaskDTO.UpdateRequest request,
                                        @Context ContainerRequestContext requestContext) {
        return updateTask(id, request, requestContext);
    }

    @PUT
    @Path("/{id}")
    public Response updateTaskPut(@PathParam("id") Long id,
                                  @Valid TaskDTO.UpdateRequest request,
                                  @Context ContainerRequestContext requestContext) {
        return updateTask(id, request, requestContext);
    }

    @PUT
    @Path("/{id}/")
    public Response updateTaskPutWithSlash(@PathParam("id") Long id,
                                           @Valid TaskDTO.UpdateRequest request,
                                           @Context ContainerRequestContext requestContext) {
        return updateTask(id, request, requestContext);
    }

    @DELETE
    @Path("/{id}")
    public Response deleteTask(@PathParam("id") Long id,
                               @Context ContainerRequestContext requestContext) {
        User user = (User) requestContext.getProperty("currentUser");
        if (user == null) {
            return Response.status(Response.Status.UNAUTHORIZED)
                    .entity(new ErrorDTO("detail", "Authentication credentials were not provided."))
                    .build();
        }

        boolean deleted = taskService.deleteTask(id, user);
        if (!deleted) {
            return Response.status(Response.Status.NOT_FOUND)
                    .entity(new ErrorDTO("detail", "Task not found."))
                    .build();
        }

        return Response.noContent().build();
    }

    @DELETE
    @Path("/{id}/")
    public Response deleteTaskWithSlash(@PathParam("id") Long id,
                                        @Context ContainerRequestContext requestContext) {
        return deleteTask(id, requestContext);
    }
}
