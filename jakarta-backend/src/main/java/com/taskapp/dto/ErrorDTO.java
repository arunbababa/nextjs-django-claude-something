package com.taskapp.dto;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class ErrorDTO {
    private String detail;
    private Map<String, List<String>> errors;

    public ErrorDTO() {
        this.errors = new HashMap<>();
    }

    public ErrorDTO(String detail) {
        this.detail = detail;
        this.errors = new HashMap<>();
    }

    public ErrorDTO(String field, String message) {
        this.errors = new HashMap<>();
        this.errors.put(field, List.of(message));
    }

    public ErrorDTO addError(String field, String message) {
        if (this.errors == null) {
            this.errors = new HashMap<>();
        }
        this.errors.put(field, List.of(message));
        return this;
    }

    // Getters and Setters
    public String getDetail() {
        return detail;
    }

    public void setDetail(String detail) {
        this.detail = detail;
    }

    public Map<String, List<String>> getErrors() {
        return errors;
    }

    public void setErrors(Map<String, List<String>> errors) {
        this.errors = errors;
    }
}
