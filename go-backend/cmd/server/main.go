package main

import (
	"log"
	"net/http"

	"go-backend/internal/config"
	"go-backend/internal/database"
	"go-backend/internal/handlers"
	"go-backend/internal/middleware"

	"github.com/gin-gonic/gin"
)

func main() {
	// 設定読み込み
	cfg := config.Load()

	// デバッグモード設定
	if !cfg.Debug {
		gin.SetMode(gin.ReleaseMode)
	}

	// データベース初期化
	if err := database.Initialize(cfg); err != nil {
		log.Fatalf("Failed to initialize database: %v", err)
	}

	// ルーター設定
	router := gin.Default()

	// CORSミドルウェア
	router.Use(middleware.CORSMiddleware(cfg))

	// ヘルスチェック
	router.GET("/health", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"status":  "healthy",
			"backend": "go",
		})
	})

	// APIルートグループ
	api := router.Group("/api")
	{
		// 認証エンドポイント（公開）
		auth := api.Group("/auth")
		{
			auth.POST("/register", handlers.Register)
			auth.POST("/login", handlers.Login)
			auth.POST("/logout", middleware.TokenAuthMiddleware(), handlers.Logout)
		}

		// ユーザーエンドポイント（認証必須）
		users := api.Group("/users")
		users.Use(middleware.TokenAuthMiddleware())
		{
			users.GET("/me", handlers.GetCurrentUser)
		}

		// タスクエンドポイント（認証必須）
		tasks := api.Group("/tasks")
		tasks.Use(middleware.TokenAuthMiddleware())
		{
			tasks.GET("", handlers.ListTasks)
			tasks.GET("/", handlers.ListTasks)
			tasks.POST("", handlers.CreateTask)
			tasks.POST("/", handlers.CreateTask)
			tasks.GET("/:id", handlers.GetTask)
			tasks.GET("/:id/", handlers.GetTask)
			tasks.PATCH("/:id", handlers.UpdateTask)
			tasks.PATCH("/:id/", handlers.UpdateTask)
			tasks.PUT("/:id", handlers.UpdateTask)
			tasks.PUT("/:id/", handlers.UpdateTask)
			tasks.DELETE("/:id", handlers.DeleteTask)
			tasks.DELETE("/:id/", handlers.DeleteTask)
		}
	}

	// サーバー起動
	log.Printf("Starting Go backend server on port %s", cfg.Port)
	log.Printf("Debug mode: %v", cfg.Debug)
	log.Printf("CORS allowed origins: %v", cfg.CORSAllowedOrigins)

	if err := router.Run(":" + cfg.Port); err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}
}
