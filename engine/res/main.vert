#version 420 core

// ── Vertex attributes (match Mesh::setup_mesh attrib locations) ──────────────
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec3 aNormal;
layout(location = 2) in vec2 aTexCoords;

// ── Uniforms ─────────────────────────────────────────────────────────────────
/// Combined Model-View-Projection (uploaded by Scene::draw / Renderer::run).
uniform mat4 uMVP;
/// Model matrix alone – needed to transform normals into world space.
uniform mat4 uModel;

// ── Outputs to fragment shader ────────────────────────────────────────────────
out vec3 vWorldPos;
out vec3 vNormal;
out vec2 vTexCoords;

void main() {
    vec4 worldPos  = uModel * vec4(aPos, 1.0);
    vWorldPos      = worldPos.xyz;

    // Normal matrix = transpose(inverse(uModel)).
    // For uniform-scale meshes mat3(uModel) is sufficient and cheaper.
    vNormal        = normalize(mat3(transpose(inverse(uModel))) * aNormal);

    vTexCoords     = aTexCoords;
    gl_Position    = uMVP * vec4(aPos, 1.0);
}
