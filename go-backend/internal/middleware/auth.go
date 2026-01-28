package middleware

import (
	"net/http"
	"strings"

	"go-backend/internal/database"
	"go-backend/internal/models"

	"github.com/gin-gonic/gin"
)

// TokenAuthMiddleware はトークン認証を行うミドルウェア
func TokenAuthMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		authHeader := c.GetHeader("Authorization")

		if authHeader == "" {
			c.JSON(http.StatusUnauthorized, gin.H{
				"detail": "Authentication credentials were not provided.",
			})
			c.Abort()
			return
		}

		// "Token <key>" フォーマットをパース
		parts := strings.SplitN(authHeader, " ", 2)
		if len(parts) != 2 || parts[0] != "Token" {
			c.JSON(http.StatusUnauthorized, gin.H{
				"detail": "Invalid token header. No credentials provided.",
			})
			c.Abort()
			return
		}

		tokenKey := parts[1]

		// トークンを検証
		var token models.Token
		result := database.GetDB().Where("key = ?", tokenKey).Preload("User").First(&token)
		if result.Error != nil {
			c.JSON(http.StatusUnauthorized, gin.H{
				"detail": "Invalid token.",
			})
			c.Abort()
			return
		}

		// ユーザー情報をコンテキストに設定
		c.Set("user", &token.User)
		c.Set("userID", token.UserID)
		c.Set("token", &token)

		c.Next()
	}
}

// GetCurrentUser はコンテキストからユーザーを取得するヘルパー
func GetCurrentUser(c *gin.Context) *models.User {
	user, exists := c.Get("user")
	if !exists {
		return nil
	}
	return user.(*models.User)
}

// GetCurrentUserID はコンテキストからユーザーIDを取得するヘルパー
func GetCurrentUserID(c *gin.Context) uint {
	userID, exists := c.Get("userID")
	if !exists {
		return 0
	}
	return userID.(uint)
}
