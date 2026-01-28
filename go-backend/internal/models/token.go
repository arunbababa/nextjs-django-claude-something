package models

import (
	"crypto/rand"
	"encoding/hex"
	"time"

	"gorm.io/gorm"
)

type Token struct {
	ID        uint           `gorm:"primarykey" json:"id"`
	Key       string         `gorm:"uniqueIndex;size:40;not null" json:"key"`
	UserID    uint           `gorm:"uniqueIndex;not null" json:"user_id"`
	User      User           `gorm:"constraint:OnDelete:CASCADE;" json:"-"`
	CreatedAt time.Time      `json:"created_at"`
	DeletedAt gorm.DeletedAt `gorm:"index" json:"-"`
}

// 新しいトークンキーを生成
func GenerateTokenKey() (string, error) {
	bytes := make([]byte, 20)
	if _, err := rand.Read(bytes); err != nil {
		return "", err
	}
	return hex.EncodeToString(bytes), nil
}

// トークンを取得または作成
func GetOrCreateToken(db *gorm.DB, userID uint) (*Token, error) {
	var token Token
	result := db.Where("user_id = ?", userID).First(&token)

	if result.Error == gorm.ErrRecordNotFound {
		key, err := GenerateTokenKey()
		if err != nil {
			return nil, err
		}

		token = Token{
			Key:    key,
			UserID: userID,
		}

		if err := db.Create(&token).Error; err != nil {
			return nil, err
		}
	} else if result.Error != nil {
		return nil, result.Error
	}

	return &token, nil
}
