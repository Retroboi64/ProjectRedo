#version 420 core

// ── Inputs from vertex shader ─────────────────────────────────────────────────
in vec3 vWorldPos;
in vec3 vNormal;
in vec2 vTexCoords;

// ── Uniforms ──────────────────────────────────────────────────────────────────
/// Camera position in world space (for specular).
uniform vec3 uCamPos;

/// Directional light direction (points *toward* the light, normalised).
uniform vec3  uLightDir;
uniform vec3  uLightColor;

/// Simple material tint – set to vec3(1.0) for a default white mesh.
uniform vec3  uBaseColor;

// ── Output ────────────────────────────────────────────────────────────────────
out vec4 FragColor;

void main() {
    vec3 N = normalize(vNormal);
    vec3 L = normalize(uLightDir);
    vec3 V = normalize(uCamPos - vWorldPos);
    vec3 H = normalize(L + V);           // half-vector for Blinn-Phong

    // Ambient
    float ambientStr  = 0.15;
    vec3  ambient     = ambientStr * uLightColor;

    // Diffuse (Lambertian)
    float diffuseStr  = max(dot(N, L), 0.0);
    vec3  diffuse     = diffuseStr * uLightColor;

    // Specular (Blinn-Phong)
    float shininess   = 32.0;
    float specularStr = pow(max(dot(N, H), 0.0), shininess);
    vec3  specular    = 0.5 * specularStr * uLightColor;

    vec3 color = (ambient + diffuse + specular) * uBaseColor;

    // Simple gamma correction
    color = pow(color, vec3(1.0 / 2.2));

    FragColor = vec4(color, 1.0);
}
