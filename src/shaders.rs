/*
    Interface publique du module shaders

    Sert à compiler les shaders.
*/

// Représente un programme de shaders OpenGL
pub struct ProgrammeOpenGL {

    pub programme: glium::Program,
}

impl ProgrammeOpenGL {

    pub fn new(affichage: &glium::Display) -> ProgrammeOpenGL {

        let vertex_shader = code_source::vertex_shader();
        let fragment_shader = code_source::fragment_shader();
        let programme = glium::Program::from_source(
            affichage,
            &vertex_shader,
            &fragment_shader,
            None);

        // On vérifie si le programme est correct, sinon on arrête le programme avec l'erreur
        let programme = match programme {

            Ok(o) => o,
            Err(e) => {
                
                match e {
                    
                    glium::program::ProgramCreationError::CompilationError(e) => {
                        println!("\n\nIl y a au moins une erreur de compilation des shaders:\n{}", e);
                    },

                    glium::program::ProgramCreationError::LinkingError(e) => {
                        println!("\n\nIl y a au moins une erreur de linking des shaders:\n{}", e);
                    },

                    _ => { 
                        println!("\n\nUne erreur inconnue est survenue à la compilation des shaders.");
                        println!("Voir glium::program::ProgramCreationError\n");
                    },
                }

                panic!("La compilation des shaders a échouée");
            }
        };

        ProgrammeOpenGL {
            
            programme: programme 
        }
    }
}





/*
    Partie privée du module shaders
*/

mod code_source
{
    // Déclaration de tous les shaders utilisés
    // La notation r#""# permet de préserver la chaîne brute

    pub fn vertex_shader() -> std::string::String
    {
        std::string::String::from(r#"
            #version 430
            uniform layout(std140);

            uniform mat4 camera_perspective;
            uniform vec3 direction_regard;

            const uint NBR_LUMIERES = 256;

            layout(std140) buffer lumieres {
                vec4 positions[NBR_LUMIERES];
                vec4 couleurs[NBR_LUMIERES];
            };
            
            in vec3 position;
            in vec3 normale;
            in vec3 coordonnees_texture;
            
            out vec3 normal;
            out vec3 coord_tex;
            
            out vec3 directionRegard;
            
            out vec3 directionLumiere;
            out float distance;

            const uint NBR_LUMIERES_PROCHES = 8;

            out LumieresProches {
                float distances[NBR_LUMIERES_PROCHES];
                vec3 directions[NBR_LUMIERES_PROCHES];
                vec4 couleurs[NBR_LUMIERES_PROCHES];
            } lumieresProches;

            void main() {
                gl_Position = camera_perspective * vec4(position, 1.0);
                
                normal = normale;

                coord_tex = coordonnees_texture;

                directionRegard = direction_regard;
                
                directionLumiere = normalize(position - vec3(positions[0]));
                distance = distance(vec3(positions[0]), position);

                // initialisation des lumieres
                uint lumieresChoisis[8];
                for(int i=0; i<NBR_LUMIERES_PROCHES; ++i) {
                    lumieresProches.distances[i] = 1000000.0;
                    //lumieresProches.directions[i] = vec3(0.0, 0.0, 0.0);
                    //lumieresProches.couleurs[i] = vec4(0.0, 0.0, 0.0, 1.0);
                    lumieresChoisis[i] = 0;
                }

                for(int j=0; j<NBR_LUMIERES; ++j) {
                    
                    float distance = distance(vec3(positions[j]), position);
                    float maximum = 0.0;
                    int index = 0;

                    for(int i=0; i<NBR_LUMIERES_PROCHES; ++i) {
                        if (maximum < lumieresProches.distances[i]) {
                            maximum = lumieresProches.distances[i];
                            index = i;
                        }
                    }

                    lumieresChoisis[index] = j;
                    /*
                    if (distance < maximum) {
                        lumieresProches.distances[index] = distance;
                        lumieresProches.directions[index] = normalize(position - vec3(positions[j]));
                        lumieresProches.couleurs[index] = couleurs[j];
                    }*/
                }

                for(int i=0; i<NBR_LUMIERES_PROCHES; ++i) {
                    lumieresProches.distances[i] = distance(vec3(positions[lumieresChoisis[i]]), position);
                    lumieresProches.directions[i] = normalize(position - vec3(positions[lumieresChoisis[i]]));
                    lumieresProches.couleurs[i] = couleurs[lumieresChoisis[i]];
                }
            }
        "#)
    }
    
    pub fn fragment_shader() -> std::string::String
    {
        std::string::String::from(r#"
            #version 430
            uniform layout(std140);

            uniform sampler2DArray textures;

            in vec3 normal;
            in vec3 coord_tex;

            in vec3 directionRegard;
            in vec3 directionLumiere;

            in float distance;

            out vec4 couleur;
            
            void main() {



                vec3 direction_reflexion = reflect(-directionLumiere, normal);
                const float intensite_speculaire = 0.45;
                float lumiere_speculaire = intensite_speculaire * pow(max(dot(directionRegard, direction_reflexion), 0.0), 12);

                const float intensite_diffuse = 0.45;
                float lumiere_diffuse = intensite_diffuse * max(dot(normal, directionLumiere), 0.0);

                const float intensite_ambiante = 1.0 - (intensite_diffuse + intensite_speculaire);
                
                const float FACTEUR_DIMINUTION = 5.0;
                float diminution = (FACTEUR_DIMINUTION * (distance + distance * distance) + 1.0);
                vec4 luminosite = (lumiere_speculaire + lumiere_diffuse + intensite_ambiante) * vec4(1.0, 0.8, 0.5, 1.0) / diminution;
               
                couleur = texture(textures, coord_tex) * luminosite;// / (2.0 * distance * distance + 1.0);
            }
        "#)
    }
}