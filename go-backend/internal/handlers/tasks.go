package handlers

import (
	"net/http"
	"strconv"

	"go-backend/internal/database"
	"go-backend/internal/middleware"
	"go-backend/internal/models"

	"github.com/gin-gonic/gin"
)

// ListTasks はユーザーのタスク一覧を返す
func ListTasks(c *gin.Context) {
	userID := middleware.GetCurrentUserID(c)
	if userID == 0 {
		c.JSON(http.StatusUnauthorized, gin.H{
			"detail": "Authentication credentials were not provided.",
		})
		return
	}

	db := database.GetDB()

	var tasks []models.Task
	if err := db.Where("user_id = ?", userID).Order("created_at DESC").Find(&tasks).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"detail": "Failed to retrieve tasks.",
		})
		return
	}

	// レスポンス形式に変換
	response := make([]models.TaskResponse, len(tasks))
	for i, task := range tasks {
		response[i] = task.ToResponse()
	}

	c.JSON(http.StatusOK, response)
}

// CreateTask は新しいタスクを作成
func CreateTask(c *gin.Context) {
	userID := middleware.GetCurrentUserID(c)
	if userID == 0 {
		c.JSON(http.StatusUnauthorized, gin.H{
			"detail": "Authentication credentials were not provided.",
		})
		return
	}

	var req models.CreateTaskRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"detail": "Invalid request data.",
			"errors": err.Error(),
		})
		return
	}

	db := database.GetDB()

	task := models.Task{
		Title:       req.Title,
		Description: req.Description,
		Completed:   req.Completed,
		UserID:      userID,
	}

	if err := db.Create(&task).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"detail": "Failed to create task.",
		})
		return
	}

	c.JSON(http.StatusCreated, task.ToResponse())
}

// GetTask は特定のタスクを取得
func GetTask(c *gin.Context) {
	userID := middleware.GetCurrentUserID(c)
	if userID == 0 {
		c.JSON(http.StatusUnauthorized, gin.H{
			"detail": "Authentication credentials were not provided.",
		})
		return
	}

	taskID, err := strconv.ParseUint(c.Param("id"), 10, 32)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"detail": "Invalid task ID.",
		})
		return
	}

	db := database.GetDB()

	var task models.Task
	if err := db.Where("id = ? AND user_id = ?", taskID, userID).First(&task).Error; err != nil {
		c.JSON(http.StatusNotFound, gin.H{
			"detail": "Task not found.",
		})
		return
	}

	c.JSON(http.StatusOK, task.ToResponse())
}

// UpdateTask はタスクを更新（部分更新対応）
func UpdateTask(c *gin.Context) {
	userID := middleware.GetCurrentUserID(c)
	if userID == 0 {
		c.JSON(http.StatusUnauthorized, gin.H{
			"detail": "Authentication credentials were not provided.",
		})
		return
	}

	taskID, err := strconv.ParseUint(c.Param("id"), 10, 32)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"detail": "Invalid task ID.",
		})
		return
	}

	db := database.GetDB()

	// タスク取得
	var task models.Task
	if err := db.Where("id = ? AND user_id = ?", taskID, userID).First(&task).Error; err != nil {
		c.JSON(http.StatusNotFound, gin.H{
			"detail": "Task not found.",
		})
		return
	}

	var req models.UpdateTaskRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"detail": "Invalid request data.",
			"errors": err.Error(),
		})
		return
	}

	// 部分更新
	updates := make(map[string]interface{})
	if req.Title != nil {
		updates["title"] = *req.Title
	}
	if req.Description != nil {
		updates["description"] = *req.Description
	}
	if req.Completed != nil {
		updates["completed"] = *req.Completed
	}

	if len(updates) > 0 {
		if err := db.Model(&task).Updates(updates).Error; err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{
				"detail": "Failed to update task.",
			})
			return
		}
	}

	// 更新後のタスクを取得
	db.First(&task, taskID)

	c.JSON(http.StatusOK, task.ToResponse())
}

// DeleteTask はタスクを削除
func DeleteTask(c *gin.Context) {
	userID := middleware.GetCurrentUserID(c)
	if userID == 0 {
		c.JSON(http.StatusUnauthorized, gin.H{
			"detail": "Authentication credentials were not provided.",
		})
		return
	}

	taskID, err := strconv.ParseUint(c.Param("id"), 10, 32)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"detail": "Invalid task ID.",
		})
		return
	}

	db := database.GetDB()

	// タスクの存在確認
	var task models.Task
	if err := db.Where("id = ? AND user_id = ?", taskID, userID).First(&task).Error; err != nil {
		c.JSON(http.StatusNotFound, gin.H{
			"detail": "Task not found.",
		})
		return
	}

	// 削除（物理削除）
	if err := db.Unscoped().Delete(&task).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"detail": "Failed to delete task.",
		})
		return
	}

	c.Status(http.StatusNoContent)
}
