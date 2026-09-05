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

    // Reflejo especular Blinn-Phong: depende de la luz, la normal y la camara.
    vec3 directionToView = normalize(viewPosition - fragPosition);
    vec3 halfDirection = normalize(directionToLight + directionToView);
    float specular = 0.0;

    if (diffuse > 0.0)
    {
        specular = pow(max(dot(normal, halfDirection), 0.0), shininess);
    }

    vec3 diffuseAndAmbient = textureColor.rgb *
                             (ambientColor + lightColor * diffuse);
    // El reflejo se agrega en blanco, como el destello sobre pintura pulida.
    vec3 specularLight = vec3(1.0) * specular * specularStrength;

    finalColor = vec4(min(diffuseAndAmbient + specularLight, vec3(1.0)),
                      textureColor.a);
}
