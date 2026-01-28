package models

import (
	"time"

	"gorm.io/gorm"
)

type Task struct {
	ID          uint           `gorm:"primarykey" json:"id"`
	Title       string         `gorm:"size:200;not null" json:"title"`
	Description string         `gorm:"type:text" json:"description"`
	Completed   bool           `gorm:"default:false" json:"completed"`
	CreatedAt   time.Time      `json:"created_at"`
	UpdatedAt   time.Time      `json:"updated_at"`
	DeletedAt   gorm.DeletedAt `gorm:"index" json:"-"`
	UserID      uint           `gorm:"not null" json:"user"`
	User        User           `gorm:"constraint:OnDelete:CASCADE;" json:"-"`
}

// タスク作成リクエスト
type CreateTaskRequest struct {
	Title       string `json:"title" binding:"required,max=200"`
	Description string `json:"description"`
	Completed   bool   `json:"completed"`
}

// タスク更新リクエスト
type UpdateTaskRequest struct {
	Title       *string `json:"title" binding:"omitempty,max=200"`
	Description *string `json:"description"`
	Completed   *bool   `json:"completed"`
}

// タスクレスポンス
type TaskResponse struct {
	ID          uint      `json:"id"`
	Title       string    `json:"title"`
	Description string    `json:"description"`
	Completed   bool      `json:"completed"`
	CreatedAt   time.Time `json:"created_at"`
	UpdatedAt   time.Time `json:"updated_at"`
	User        uint      `json:"user"`
}

func (t *Task) ToResponse() TaskResponse {
	return TaskResponse{
		ID:          t.ID,
		Title:       t.Title,
		Description: t.Description,
		Completed:   t.Completed,
		CreatedAt:   t.CreatedAt,
		UpdatedAt:   t.UpdatedAt,
		User:        t.UserID,
	}
}
