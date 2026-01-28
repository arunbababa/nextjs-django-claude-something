package com.taskapp.repository;

import com.taskapp.entity.Token;
import com.taskapp.entity.User;
import com.taskapp.util.TokenUtil;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;
import jakarta.persistence.EntityManager;
import jakarta.persistence.NoResultException;
import jakarta.persistence.PersistenceContext;
import jakarta.transaction.Transactional;

import java.util.Optional;

@ApplicationScoped
public class TokenRepository {

    @PersistenceContext
    private EntityManager em;

    @Inject
    private TokenUtil tokenUtil;

    @Transactional
    public Token save(Token token) {
        if (token.getId() == null) {
            em.persist(token);
            return token;
        } else {
            return em.merge(token);
        }
    }

    public Optional<Token> findByKey(String key) {
        try {
            Token token = em.createNamedQuery("Token.findByKey", Token.class)
                    .setParameter("key", key)
                    .getSingleResult();
            return Optional.of(token);
        } catch (NoResultException e) {
            return Optional.empty();
        }
    }

    public Optional<Token> findByUserId(Long userId) {
        try {
            Token token = em.createNamedQuery("Token.findByUser", Token.class)
                    .setParameter("userId", userId)
                    .getSingleResult();
            return Optional.of(token);
        } catch (NoResultException e) {
            return Optional.empty();
        }
    }

    @Transactional
    public Token getOrCreate(User user) {
        Optional<Token> existingToken = findByUserId(user.getId());
        if (existingToken.isPresent()) {
            return existingToken.get();
        }

        String key = tokenUtil.generateToken();
        Token token = new Token(key, user);
        em.persist(token);
        return token;
    }

    @Transactional
    public void delete(Token token) {
        em.remove(em.contains(token) ? token : em.merge(token));
    }

    @Transactional
    public void deleteByUserId(Long userId) {
        em.createNamedQuery("Token.deleteByUser")
                .setParameter("userId", userId)
                .executeUpdate();
    }
}
