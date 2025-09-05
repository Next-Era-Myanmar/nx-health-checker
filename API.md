# NX Health Checker API Documentation

## Overview

The NX Health Checker provides a RESTful API for managing service health monitoring. All endpoints return JSON responses and use standard HTTP status codes.

## Base URL

```
http://localhost:3030
```

## Authentication

Most endpoints require authentication via session cookies. The login endpoint returns a session cookie that should be included in subsequent requests.

## Endpoints

### Authentication

#### Login
```http
POST /login
Content-Type: application/x-www-form-urlencoded

username=admin&password=admin
```

**Response:**
```json
{
  "success": true,
  "message": "Login successful!",
  "redirect_url": "/dashboard"
}
```

#### Logout
```http
GET /logout
```

**Response:** Redirects to home page

#### Change Password
```http
POST /api/change-password
Content-Type: application/json

{
  "current_password": "old_password",
  "new_password": "new_password",
  "confirm_password": "new_password"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Password changed successfully!"
}
```

### Service Management

#### Get All Services
```http
GET /api/services
```

**Response:**
```json
[
  {
    "id": 1,
    "service_name": "My API",
    "healthcheck_url": "https://api.example.com/health",
    "healthcheck_duration_seconds": 30,
    "created_at": "2025-09-04T04:00:00Z",
    "updated_at": "2025-09-04T04:00:00Z"
  }
]
```

#### Create Service
```http
POST /api/services
Content-Type: application/json

{
  "service_name": "My API",
  "healthcheck_url": "https://api.example.com/health",
  "healthcheck_duration_seconds": 30
}
```

**Response:**
```json
{
  "success": true,
  "message": "Service created successfully",
  "id": 1
}
```

#### Update Service
```http
PUT /api/services/{id}
Content-Type: application/json

{
  "service_name": "Updated API",
  "healthcheck_url": "https://api.example.com/health",
  "healthcheck_duration_seconds": 60
}
```

**Response:**
```json
{
  "success": true,
  "message": "Service updated successfully"
}
```

#### Delete Service
```http
DELETE /api/services/{id}
```

**Response:**
```json
{
  "success": true,
  "message": "Service deleted successfully"
}
```

### Health Checks

#### Application Health Check
```http
GET /health
```

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2025-09-04T04:00:00Z",
  "service": "nx-health-checker"
}
```

#### Check Single Service
```http
GET /api/services/{id}/health
```

**Response:**
```json
{
  "service_id": 1,
  "service_name": "My API",
  "status": "UP",
  "checked_at": "2025-09-04T04:00:00Z"
}
```

#### Check All Services
```http
GET /api/services/health
```

**Response:**
```json
{
  "services": [
    {
      "service_id": 1,
      "service_name": "My API",
      "status": "UP",
      "checked_at": "2025-09-04T04:00:00Z"
    }
  ],
  "total_checked": 1
}
```

### Metrics

#### Prometheus Metrics
```http
GET /metrics
```

**Response:** Prometheus-formatted metrics

#### Restart Collectors
```http
POST /api/metrics/restart
```

**Response:**
```json
{
  "success": true,
  "message": "Collectors restarted"
}
```

## Error Responses

### 400 Bad Request
```json
{
  "success": false,
  "message": "Invalid request data"
}
```

### 401 Unauthorized
```json
{
  "success": false,
  "message": "Unauthorized"
}
```

### 404 Not Found
```json
{
  "success": false,
  "message": "Service not found"
}
```

### 500 Internal Server Error
```json
{
  "success": false,
  "message": "Internal server error"
}
```

## Status Codes

- **UP**: Service is healthy (HTTP 200 response)
- **DOWN**: Service is unhealthy (non-200 response or connection error)
- **Checking...**: Health check in progress

## Rate Limiting

Currently, there are no rate limits implemented. This may be added in future versions.

## CORS

The API supports CORS for cross-origin requests.

## Examples

### cURL Examples

#### Login
```bash
curl -X POST http://localhost:3030/login \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=admin&password=admin" \
  -c cookies.txt
```

#### Create Service
```bash
curl -X POST http://localhost:3030/api/services \
  -H "Content-Type: application/json" \
  -b cookies.txt \
  -d '{
    "service_name": "My API",
    "healthcheck_url": "https://api.example.com/health",
    "healthcheck_duration_seconds": 30
  }'
```

#### Check Health
```bash
curl -X GET http://localhost:3030/health
```

### JavaScript Examples

#### Login
```javascript
const response = await fetch('/login', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/x-www-form-urlencoded',
  },
  body: 'username=admin&password=admin',
  credentials: 'include'
});

const result = await response.json();
console.log(result);
```

#### Create Service
```javascript
const response = await fetch('/api/services', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  credentials: 'include',
  body: JSON.stringify({
    service_name: 'My API',
    healthcheck_url: 'https://api.example.com/health',
    healthcheck_duration_seconds: 30
  })
});

const result = await response.json();
console.log(result);
```

## WebSocket Support

WebSocket support is not currently implemented but may be added in future versions for real-time updates.

## API Versioning

The current API version is v1. Future versions will be available at `/api/v2/`, etc.

## Changelog

### v1.0.0
- Initial API release
- Service management endpoints
- Health check endpoints
- Authentication system
- Prometheus metrics integration
