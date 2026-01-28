package com.taskapp.dto;

import com.taskapp.entity.Task;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;
import java.time.LocalDateTime;

public class TaskDTO {

    // Task Response
    public static class TaskResponse {
        private Long id;
        private String title;
        private String description;
        private Boolean completed;
        private LocalDateTime createdAt;
        private LocalDateTime updatedAt;
        private Long user;

        public TaskResponse() {
        }

        public static TaskResponse fromEntity(Task task) {
            TaskResponse dto = new TaskResponse();
            dto.id = task.getId();
            dto.title = task.getTitle();
            dto.description = task.getDescription();
            dto.completed = task.getCompleted();
            dto.createdAt = task.getCreatedAt();
            dto.updatedAt = task.getUpdatedAt();
            dto.user = task.getUser().getId();
            return dto;
        }

        // Getters and Setters
        public Long getId() {
            return id;
        }

        public void setId(Long id) {
            this.id = id;
        }

        public String getTitle() {
            return title;
        }

        public void setTitle(String title) {
            this.title = title;
        }

        public String getDescription() {
            return description;
        }

        public void setDescription(String description) {
            this.description = description;
        }

        public Boolean getCompleted() {
            return completed;
        }

        public void setCompleted(Boolean completed) {
            this.completed = completed;
        }

        public LocalDateTime getCreatedAt() {
            return createdAt;
        }

        public void setCreatedAt(LocalDateTime createdAt) {
            this.createdAt = createdAt;
        }

        public LocalDateTime getUpdatedAt() {
            return updatedAt;
        }

        public void setUpdatedAt(LocalDateTime updatedAt) {
            this.updatedAt = updatedAt;
        }

        public Long getUser() {
            return user;
        }

        public void setUser(Long user) {
            this.user = user;
        }
    }

    // Create Task Request
    public static class CreateRequest {
        @NotBlank(message = "Title is required")
        @Size(max = 200, message = "Title must be at most 200 characters")
        private String title;

        private String description;

        private Boolean completed = false;

        // Getters and Setters
        public String getTitle() {
            return title;
        }

        public void setTitle(String title) {
            this.title = title;
        }

        public String getDescription() {
            return description;
        }

        public void setDescription(String description) {
            this.description = description;
        }

        public Boolean getCompleted() {
            return completed;
        }

        public void setCompleted(Boolean completed) {
            this.completed = completed;
        }
    }

    // Update Task Request
    public static class UpdateRequest {
        @Size(max = 200, message = "Title must be at most 200 characters")
        private String title;

        private String description;

        private Boolean completed;

        // Getters and Setters
        public String getTitle() {
            return title;
        }

        public void setTitle(String title) {
            this.title = title;
        }

        public String getDescription() {
            return description;
        }

        public void setDescription(String description) {
            this.description = description;
        }

        public Boolean getCompleted() {
            return completed;
        }

        public void setCompleted(Boolean completed) {
            this.completed = completed;
        }
    }
}
