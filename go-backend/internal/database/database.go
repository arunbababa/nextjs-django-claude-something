package database

import (
	"log"
	"os"
	"strings"

	"go-backend/internal/config"
	"go-backend/internal/models"

	"gorm.io/driver/postgres"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
)

var DB *gorm.DB

func Initialize(cfg *config.Config) error {
	var dialector gorm.Dialector

	// データベースURLに基づいて接続を設定
	if cfg.DatabaseURL != "" && strings.HasPrefix(cfg.DatabaseURL, "postgres") {
		// PostgreSQL接続
		dialector = postgres.Open(cfg.DatabaseURL)
		log.Println("Using PostgreSQL database")
	} else {
		// SQLite接続（開発用デフォルト）
		dialector = sqlite.Open("db.sqlite3")
		log.Println("Using SQLite database")
	}

	// ロガー設定
	var gormLogger logger.Interface
	if cfg.Debug {
		gormLogger = logger.New(
			log.New(os.Stdout, "\r\n", log.LstdFlags),
			logger.Config{
				LogLevel: logger.Info,
			},
		)
	} else {
		gormLogger = logger.Default.LogMode(logger.Silent)
	}

	// データベース接続
	var err error
	DB, err = gorm.Open(dialector, &gorm.Config{
		Logger: gormLogger,
	})
	if err != nil {
		return err
	}

	// マイグレーション実行
	if err := runMigrations(); err != nil {
		return err
	}

	log.Println("Database initialized successfully")
	return nil
}

func runMigrations() error {
	// 自動マイグレーション
	return DB.AutoMigrate(
		&models.User{},
		&models.Task{},
		&models.Token{},
	)
}

func GetDB() *gorm.DB {
	return DB
}
