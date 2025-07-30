# Visualization Integration Troubleshooting Guide

## Common Issues and Solutions

### 1. Visualization Requests Failing with 400 Bad Request

**Symptom**: Requests to `/visualizations/:id` return 400 status code.

**Possible Causes**:
- Missing or malformed visualization context headers
- Invalid UUID in path parameter
- Missing required query parameters

**Solutions**:
1. Ensure all required headers are present:
   - `X-Originating-App`
   - `X-User-ID`
   - `X-Sharing-Scope`
   - `X-Accessibility-Mode`
   - `X-LOD-Level`

2. Verify the visualization ID is a valid UUID v4

3. Check that query parameters match expected format:
   - `width` and `height` should be positive integers
   - `lod_level` should be between 0-5

### 2. Visualization Requests Failing with 403 Forbidden

**Symptom**: Requests to visualization endpoints return 403 status code.

**Possible Causes**:
- User does not have access to the requested visualization
- Invalid sharing scope configuration
- Expired or invalid session token

**Solutions**:
1. Verify the user has permission to access the visualization
2. Check that the sharing scope allows access for the requesting user
3. Ensure the session token is valid and not expired

### 3. Slow Visualization Loading

**Symptom**: Visualizations take longer than expected to load.

**Possible Causes**:
- Cache misses resulting in regeneration
- High level of detail (LOD) settings
- Network latency to backend services
- Resource constraints on visualization service

**Solutions**:
1. Check cache hit ratios in monitoring dashboard
2. Adjust LOD settings for better performance
3. Verify network connectivity to backend services
4. Monitor resource usage on visualization service

### 4. Accessibility Metadata Missing or Incorrect

**Symptom**: Screen readers not properly announcing visualization content.

**Possible Causes**:
- Accessibility mode not properly set in request
- Missing navigation map in response
- Incorrect ARIA properties

**Solutions**:
1. Ensure `X-Accessibility-Mode` header is set correctly
2. Verify that the visualization service generates proper navigation maps
3. Check that ARIA properties are correctly applied

### 5. WebSocket Connection Failures

**Symptom**: WebSocket connections to `/visualizations/:id/ws` fail to establish.

**Possible Causes**:
- Invalid visualization ID
- User lacks access permissions
- Backend service not available
- Network configuration blocking WebSocket upgrades

**Solutions**:
1. Verify the visualization ID exists and is accessible
2. Check user permissions for the visualization
3. Ensure the BI Analytics service is running
4. Verify network configuration allows WebSocket connections

## Debugging Steps

### 1. Check Gateway Logs

Look for error messages in the API gateway logs:

```bash
# If running locally
tail -f /var/log/cpc/api-gateway.log

# If running in Docker
docker logs cpc-api-gateway
```

### 2. Verify Context Headers

Use a tool like curl to verify headers are correctly set:

```bash
curl -H "X-Originating-App: dashboard" \
     -H "X-User-ID: 123e4567-e89b-12d3-a456-426614174000" \
     -H "X-Sharing-Scope: public" \
     -H "X-Accessibility-Mode: screen_reader" \
     -H "X-LOD-Level: 2" \
     http://localhost:3001/visualizations/123e4567-e89b-12d3-a456-426614174000
```

### 3. Check Cache Status

Verify if requests are being served from cache:

```bash
curl -v http://localhost:3001/visualizations/123e4567-e89b-12d3-a456-426614174000/image
```

Look for `X-Cache: HIT` in the response headers.

### 4. Monitor Performance Metrics

Check the monitoring dashboard for:
- Request latency trends
- Cache hit ratios
- Error rates by endpoint
- Resource utilization

## Performance Optimization

### 1. Cache Configuration

Adjust cache TTL settings based on data update frequency:
- Static visualizations: 1 hour
- Dynamic visualizations: 5 minutes
- Real-time visualizations: 30 seconds

### 2. Level of Detail (LOD) Settings

Configure appropriate LOD levels per app:
- Dashboard: LOD 2 (balanced)
- Reporting: LOD 3 (high detail)
- Collaboration: LOD 1 (low detail for performance)

### 3. Progressive Loading

Implement progressive loading in client applications:
1. Load low-resolution preview first
2. Stream higher detail in background
3. Establish WebSocket connection for updates

## Contact Support

If issues persist after following this guide, contact the CPC development team:

- **Email**: dev-support@cpc.coop
- **Slack**: #visualization-support channel
- **Documentation**: https://docs.cpc.coop/visualization