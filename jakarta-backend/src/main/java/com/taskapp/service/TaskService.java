package com.taskapp.service;

import com.taskapp.dto.TaskDTO;
import com.taskapp.entity.Task;
import com.taskapp.entity.User;
import com.taskapp.repository.TaskRepository;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;
import jakarta.transaction.Transactional;

import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;

@ApplicationScoped
public class TaskService {

    @Inject
    private TaskRepository taskRepository;

    public List<TaskDTO.TaskResponse> getTasksByUser(User user) {
        return taskRepository.findByUserId(user.getId())
                .stream()
                .map(TaskDTO.TaskResponse::fromEntity)
                .collect(Collectors.toList());
    }

    public Optional<TaskDTO.TaskResponse> getTaskByIdAndUser(Long id, User user) {
        return taskRepository.findByIdAndUserId(id, user.getId())
                .map(TaskDTO.TaskResponse::fromEntity);
    }

    @Transactional
    public TaskDTO.TaskResponse createTask(TaskDTO.CreateRequest request, User user) {
        Task task = new Task();
        task.setTitle(request.getTitle());
        task.setDescription(request.getDescription() != null ? request.getDescription() : "");
        task.setCompleted(request.getCompleted() != null ? request.getCompleted() : false);
        task.setUser(user);

        task = taskRepository.save(task);
        return TaskDTO.TaskResponse.fromEntity(task);
    }

    @Transactional
    public Optional<TaskDTO.TaskResponse> updateTask(Long id, TaskDTO.UpdateRequest request, User user) {
        Optional<Task> taskOpt = taskRepository.findByIdAndUserId(id, user.getId());

        if (taskOpt.isEmpty()) {
            return Optional.empty();
        }

        Task task = taskOpt.get();

        // Partial update
        if (request.getTitle() != null) {
            task.setTitle(request.getTitle());
        }
        if (request.getDescription() != null) {
            task.setDescription(request.getDescription());
        }
        if (request.getCompleted() != null) {
            task.setCompleted(request.getCompleted());
        }

        task = taskRepository.save(task);
        return Optional.of(TaskDTO.TaskResponse.fromEntity(task));
    }

    @Transactional
    public boolean deleteTask(Long id, User user) {
        Optional<Task> taskOpt = taskRepository.findByIdAndUserId(id, user.getId());

        if (taskOpt.isEmpty()) {
            return false;
        }

        taskRepository.delete(taskOpt.get());
        return true;
    }
}
