const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8000/api';

interface User {
  id: number;
  username: string;
  email: string;
}

interface AuthResponse {
  token: string;
  user: User;
}

interface Task {
  id: number;
  title: string;
  description: string;
  completed: boolean;
  created_at: string;
  updated_at: string;
}

class ApiClient {
  private getHeaders(): HeadersInit {
    const headers: HeadersInit = {
      'Content-Type': 'application/json',
    };

    const token = localStorage.getItem('token');
    if (token) {
      headers['Authorization'] = `Token ${token}`;
    }

    return headers;
  }

  async register(username: string, email: string, password: string): Promise<AuthResponse> {
    const response = await fetch(`${API_URL}/auth/register`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, email, password, password2: password }),
    });

    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.detail || 'Registration failed');
    }

    return response.json();
  }

  async login(username: string, password: string): Promise<AuthResponse> {
    const response = await fetch(`${API_URL}/auth/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password }),
    });

    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.error || 'Login failed');
    }

    return response.json();
  }

  async logout(): Promise<void> {
    const response = await fetch(`${API_URL}/auth/logout`, {
      method: 'POST',
      headers: this.getHeaders(),
    });

    if (!response.ok) {
      throw new Error('Logout failed');
    }
  }

  async getCurrentUser(): Promise<User> {
    const response = await fetch(`${API_URL}/users/me`, {
      headers: this.getHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch user');
    }

    return response.json();
  }

  async getTasks(): Promise<Task[]> {
    const response = await fetch(`${API_URL}/tasks/`, {
      headers: this.getHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch tasks');
    }

    return response.json();
  }

  async createTask(title: string, description: string): Promise<Task> {
    const response = await fetch(`${API_URL}/tasks/`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ title, description, completed: false }),
    });

    if (!response.ok) {
      throw new Error('Failed to create task');
    }

    return response.json();
  }

  async updateTask(id: number, data: Partial<Task>): Promise<Task> {
    const response = await fetch(`${API_URL}/tasks/${id}/`, {
      method: 'PATCH',
      headers: this.getHeaders(),
      body: JSON.stringify(data),
    });

    if (!response.ok) {
      throw new Error('Failed to update task');
    }

    return response.json();
  }

  async deleteTask(id: number): Promise<void> {
    const response = await fetch(`${API_URL}/tasks/${id}/`, {
      method: 'DELETE',
      headers: this.getHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to delete task');
    }
  }
}

export const api = new ApiClient();
export type { User, Task, AuthResponse };
