package handlers

import (
	"net/http"
	"regexp"
	"strings"
	"unicode"

	"go-backend/internal/database"
	"go-backend/internal/middleware"
	"go-backend/internal/models"

	"github.com/gin-gonic/gin"
)

// RegisterRequest は登録リクエストの構造体
type RegisterRequest struct {
	Username  string `json:"username" binding:"required,min=1,max=150"`
	Email     string `json:"email" binding:"required,email"`
	Password  string `json:"password" binding:"required,min=8"`
	Password2 string `json:"password2" binding:"required"`
}

// LoginRequest はログインリクエストの構造体
type LoginRequest struct {
	Username string `json:"username" binding:"required"`
	Password string `json:"password" binding:"required"`
}

// AuthResponse は認証レスポンスの構造体
type AuthResponse struct {
	Token string               `json:"token"`
	User  models.UserResponse `json:"user"`
}

// validatePassword はパスワードの強度を検証
func validatePassword(password string) []string {
	var errors []string

	if len(password) < 8 {
		errors = append(errors, "Password must be at least 8 characters long.")
	}

	// 数字のみでないかチェック
	allDigits := true
	for _, c := range password {
		if !unicode.IsDigit(c) {
			allDigits = false
			break
		}
	}
	if allDigits {
		errors = append(errors, "Password can't be entirely numeric.")
	}

	// よくあるパスワードチェック
	commonPasswords := []string{
		"password", "12345678", "123456789", "qwerty", "abc123",
		"password1", "password123", "admin", "letmein", "welcome",
	}
	lowerPassword := strings.ToLower(password)
	for _, common := range commonPasswords {
		if lowerPassword == common {
			errors = append(errors, "Password is too common.")
			break
		}
	}

	return errors
}

// validateUsername はユーザー名を検証
func validateUsername(username string) []string {
	var errors []string

	// 英数字、アンダースコア、ハイフン、ドット、@のみ許可
	validUsername := regexp.MustCompile(`^[\w.@+-]+$`)
	if !validUsername.MatchString(username) {
		errors = append(errors, "Username may only contain letters, numbers, and @/./+/-/_ characters.")
	}

	return errors
}

// Register はユーザー登録を処理
func Register(c *gin.Context) {
	var req RegisterRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"detail": "Invalid request data.",
			"errors": err.Error(),
		})
		return
	}

	// パスワード一致チェック
	if req.Password != req.Password2 {
		c.JSON(http.StatusBadRequest, gin.H{
			"password2": []string{"Passwords don't match."},
		})
		return
	}

	// ユーザー名検証
	if usernameErrors := validateUsername(req.Username); len(usernameErrors) > 0 {
		c.JSON(http.StatusBadRequest, gin.H{
			"username": usernameErrors,
		})
		return
	}

	// パスワード検証
	if passwordErrors := validatePassword(req.Password); len(passwordErrors) > 0 {
		c.JSON(http.StatusBadRequest, gin.H{
			"password": passwordErrors,
		})
		return
	}

	db := database.GetDB()

	// ユーザー名の重複チェック
	var existingUser models.User
	if err := db.Where("username = ?", req.Username).First(&existingUser).Error; err == nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"username": []string{"A user with that username already exists."},
		})
		return
	}

	// メールアドレスの重複チェック
	if err := db.Where("email = ?", req.Email).First(&existingUser).Error; err == nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"email": []string{"A user with that email already exists."},
		})
		return
	}

	// ユーザー作成
	user := models.User{
		Username: req.Username,
		Email:    req.Email,
	}

	if err := user.SetPassword(req.Password); err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"detail": "Failed to create user.",
		})
		return
	}

	if err := db.Create(&user).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"detail": "Failed to create user.",
		})
		return
	}

	// トークン生成
	token, err := models.GetOrCreateToken(db, user.ID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"detail": "Failed to create authentication token.",
		})
		return
	}

	c.JSON(http.StatusCreated, AuthResponse{
		Token: token.Key,
		User:  user.ToResponse(),
	})
}

// Login はユーザーログインを処理
func Login(c *gin.Context) {
	var req LoginRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"detail": "Invalid request data.",
		})
		return
	}

	db := database.GetDB()

	// ユーザー検索
	var user models.User
	if err := db.Where("username = ?", req.Username).First(&user).Error; err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"detail": "Unable to log in with provided credentials.",
		})
		return
	}

	// パスワード検証
	if !user.CheckPassword(req.Password) {
		c.JSON(http.StatusBadRequest, gin.H{
			"detail": "Unable to log in with provided credentials.",
		})
		return
	}

	// トークン取得または作成
	token, err := models.GetOrCreateToken(db, user.ID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"detail": "Failed to create authentication token.",
		})
		return
	}

	c.JSON(http.StatusOK, AuthResponse{
		Token: token.Key,
		User:  user.ToResponse(),
	})
}

// Logout はユーザーログアウトを処理
func Logout(c *gin.Context) {
	token, exists := c.Get("token")
	if !exists {
		c.JSON(http.StatusUnauthorized, gin.H{
			"detail": "Authentication credentials were not provided.",
		})
		return
	}

	db := database.GetDB()

	// トークン削除
	if err := db.Delete(token).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"detail": "Failed to logout.",
		})
		return
	}

	c.JSON(http.StatusOK, gin.H{
		"detail": "Successfully logged out.",
	})
}

// GetCurrentUser は現在のユーザー情報を返す
func GetCurrentUser(c *gin.Context) {
	user := middleware.GetCurrentUser(c)
	if user == nil {
		c.JSON(http.StatusUnauthorized, gin.H{
			"detail": "Authentication credentials were not provided.",
		})
		return
	}

	c.JSON(http.StatusOK, user.ToResponse())
}
