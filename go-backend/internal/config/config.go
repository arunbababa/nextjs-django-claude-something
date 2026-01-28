package config

import (
	"os"
	"strings"

	"github.com/joho/godotenv"
)

type Config struct {
	SecretKey          string
	Debug              bool
	AllowedHosts       []string
	DatabaseURL        string
	CORSAllowedOrigins []string
	Port               string
}

func Load() *Config {
	// .envファイルを読み込む（存在する場合）
	_ = godotenv.Load()

	config := &Config{
		SecretKey:          getEnv("SECRET_KEY", "your-secret-key-change-in-production"),
		Debug:              getEnv("DEBUG", "true") == "true",
		AllowedHosts:       strings.Split(getEnv("ALLOWED_HOSTS", "localhost,127.0.0.1"), ","),
		DatabaseURL:        getEnv("DATABASE_URL", ""),
		CORSAllowedOrigins: strings.Split(getEnv("CORS_ALLOWED_ORIGINS", "http://localhost:3000,http://127.0.0.1:3000"), ","),
		Port:               getEnv("PORT", "8080"),
	}

	return config
}

func getEnv(key, defaultValue string) string {
	if value, exists := os.LookupEnv(key); exists {
		return value
	}
	return defaultValue
}
