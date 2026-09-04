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
uniform vec3 viewPosition;
uniform float shininess;
uniform float specularStrength;

out vec4 finalColor;

void main()
{
    vec4 textureColor = texture(texture0, fragTexCoord) * colDiffuse * fragColor;

    vec3 normal = normalize(fragNormal);
    vec3 directionToLight = normalize(lightPosition - fragPosition);
    float diffuse = max(dot(normal, directionToLight), 0.0);

    // Iluminacion especular Blinn-Phong. El vector intermedio apunta entre
    // la luz y la camara, y permite calcular el reflejo brillante.
    vec3 directionToView = normalize(viewPosition - fragPosition);
    vec3 halfDirection = normalize(directionToLight + directionToView);
    float specular = 0.0;

    if (diffuse > 0.0)
    {
        specular = pow(max(dot(normal, halfDirection), 0.0), shininess);
    }

    vec3 diffuseAndAmbient = textureColor.rgb *
                             (ambientColor + lightColor * diffuse);
    vec3 specularLight = lightColor * specular * specularStrength;

    finalColor = vec4(diffuseAndAmbient + specularLight, textureColor.a);
}
