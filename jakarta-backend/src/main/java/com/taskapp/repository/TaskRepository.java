package com.taskapp.repository;

import com.taskapp.entity.Task;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.persistence.EntityManager;
import jakarta.persistence.NoResultException;
import jakarta.persistence.PersistenceContext;
import jakarta.transaction.Transactional;

import java.util.List;
import java.util.Optional;

@ApplicationScoped
public class TaskRepository {

    @PersistenceContext
    private EntityManager em;

    @Transactional
    public Task save(Task task) {
        if (task.getId() == null) {
            em.persist(task);
            return task;
        } else {
            return em.merge(task);
        }
    }

    public Optional<Task> findById(Long id) {
        Task task = em.find(Task.class, id);
        return Optional.ofNullable(task);
    }

    public List<Task> findByUserId(Long userId) {
        return em.createNamedQuery("Task.findByUser", Task.class)
                .setParameter("userId", userId)
                .getResultList();
    }

    public Optional<Task> findByIdAndUserId(Long id, Long userId) {
        try {
            Task task = em.createNamedQuery("Task.findByIdAndUser", Task.class)
                    .setParameter("id", id)
                    .setParameter("userId", userId)
                    .getSingleResult();
            return Optional.of(task);
        } catch (NoResultException e) {
            return Optional.empty();
        }
    }

    @Transactional
    public void delete(Task task) {
        em.remove(em.contains(task) ? task : em.merge(task));
    }

    @Transactional
    public void deleteById(Long id) {
        Task task = em.find(Task.class, id);
        if (task != null) {
            em.remove(task);
        }
    }
}
