#version 330

in vec3 fragPosition;
in vec2 fragTexCoord;
in vec3 fragNormal;
in vec4 fragColor;

uniform sampler2D texture0;
uniform vec4 colDiffuse;

uniform vec3 lightPosition;
uniform vec3 lightColor;
uniform vec3 ambientColor;

out vec4 finalColor;

void main()
{
    vec4 textureColor = texture(texture0, fragTexCoord) * colDiffuse * fragColor;

    vec3 normal = normalize(fragNormal);
    vec3 directionToLight = normalize(lightPosition - fragPosition);
    float diffuse = max(dot(normal, directionToLight), 0.0);

    // Solo se combinan luz ambiental y luz difusa de Lambert.
    vec3 illumination = ambientColor + lightColor * diffuse;
    finalColor = vec4(textureColor.rgb * illumination, textureColor.a);
}
