package com.taskapp.entity;

import jakarta.persistence.*;
import java.time.LocalDateTime;

@Entity
@Table(name = "tokens")
@NamedQueries({
    @NamedQuery(name = "Token.findByKey", query = "SELECT t FROM Token t WHERE t.key = :key"),
    @NamedQuery(name = "Token.findByUser", query = "SELECT t FROM Token t WHERE t.user.id = :userId"),
    @NamedQuery(name = "Token.deleteByUser", query = "DELETE FROM Token t WHERE t.user.id = :userId")
})
public class Token {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    @Column(name = "token_key", unique = true, nullable = false, length = 40)
    private String key;

    @OneToOne(fetch = FetchType.EAGER)
    @JoinColumn(name = "user_id", unique = true, nullable = false)
    private User user;

    @Column(name = "created_at", nullable = false, updatable = false)
    private LocalDateTime createdAt;

    public Token() {
    }

    public Token(String key, User user) {
        this.key = key;
        this.user = user;
    }

    @PrePersist
    protected void onCreate() {
        createdAt = LocalDateTime.now();
    }

    // Getters and Setters
    public Long getId() {
        return id;
    }

    public void setId(Long id) {
        this.id = id;
    }

    public String getKey() {
        return key;
    }

    public void setKey(String key) {
        this.key = key;
    }

    public User getUser() {
        return user;
    }

    public void setUser(User user) {
        this.user = user;
    }

    public LocalDateTime getCreatedAt() {
        return createdAt;
    }
}
