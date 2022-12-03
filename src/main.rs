/*
PROYECTO
ESCUELA DE FUTBOL ATLAS RUSO
En el siguiente proyecto, utilizar las estructuras vistas en clase durante el semestre,
selectivas: simple, doble, múltiple, ciclos: while, do while y for, además de las funciones
para cadenas, funciones (con y sin retorno), arreglos y estructuras, arreglo de estructura,
funciones para cambiar color de texto y fondo y funciones de fecha y hora.
Comentar dónde se aplica cada uno. Por lo menos en una ocasión cada uno.
El proyecto se desarrolla para llevar el control de una escuela de futbol “ATLAS RUSO”,
donde el encargado de la escuela puede dar de alta a los Entrenadores y revisa los reportes
de la información generada, los adeudos, torneos activos y datos de los equipos.
*/

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{io::{self, Write}, vec, fmt::*, process::exit};
use colored::*;
use core::mem::size_of;
use rand::prelude::*;
use scanf::scanf;
use libc::fgets;

const LIM: u8 = 30;
const LIM2: u8 = 60;
const ABUELO: usize = 255;
const INVALID: &str = "\nOpción inválida";
const NTN: &str = "\nNo se ingresó un número válido";
const NTR: &str = "\nOpción no leida";

/*----------STRUCT-EQUIPO----------*/
	/*
	Los datos de un equipo son: CÓDIGO_EQ, CATEGORÍA, ENTRENADOR, HARARIO
	Los equipos pueden tener hasta 30 integrantes
	*/
pub struct Team {
    EQteamID: String,
    EQcategory: String,
    EQcoachID: String,
    EQschedule: String
}
/*----------/STRUCT-EQUIPO----------*/


/*----------STRUCT-ENTRENADOR----------*/
	/*
	Los datos del entrenador son: CÓDIGO_EN, NOMBRE, AP, AM, EDAD, ESTUDIOS,
	SUELDOXEQ, TEL.
    */
pub struct Trainer {
    TRusrname: String,
    TRpassword: String,
    TRcode: String,
    TRname: String,
    TRlastName1: String,
    TRlastName2: String,
    TRcareer: String,
    TRsalary: String,
    TRregDate: String
}

impl Trainer {
    fn to_admin() -> Trainer {
        Trainer { TRusrname: "ADMIN".to_string(), TRpassword: "1212".to_string(), TRcode: "sex".to_string(), TRname: "Jesus".to_string(), TRlastName1: "de".to_string(), TRlastName2: "Nazaret".to_string(), TRcareer: "Profeta".to_string(), TRsalary: "todo".to_string(), TRregDate: "0/0/0".to_string() }
    }
}
/*----------/STRUCT-ENTRENADOR----------*/


/*----------STRUCT-JUGADOR----------*/
	/*
	Los datos de Jugadores son: CÓDIGO_J, NOMBRE, AP, AM, TELS(SON 3 TELEFONOS,
	PADRE, CASA, EMERGENCIA),FECHA NACIMIENTO, CURP.
	*/
pub struct Player {
    PLplayerID: String,
    PLteamID: String,
    EQcoachID: String,
    PLlastname1: String,
    PLlastname2: String,
    PLphonenumber1: String,
    PLphonenumber2: String,
    PLphonenumber3: String,
    PLbday: String,
    PLcurp: String
}
/*----------/STRUCT-JUGADOR----------*/


/*----------STRUCT-PAGOS----------*/
pub struct Check {
    CHplayerID: String,
    CHcoachID: String,
    CHamount: String,
    CHdate: String,
    CHweek: String
}
/*----------/STRUCT-PAGOS----------*/


/*----------STRUCT-TORNEOS----------*/
pub struct Tourn{
	TOname: String,
	TOdate: String,
	TOcat: String,
	TOcoachID: String,
	TOcost: String,
	TOgamesDone: String,
	TOgamesWon: String,
	TOgamesLost: String,
	TOgamesTied: String
}
/*----------/STRUCT-TORNEOS----------*/


fn main() {
    let mut equipos: Vec<Team> = Vec::with_capacity(ABUELO);
    let mut entrenadores: Vec<Trainer> = Vec::with_capacity(ABUELO);
    let mut jugadores: Vec<Player> = Vec::with_capacity(ABUELO);

    entrenadores.push(Trainer::to_admin());

    inicio(&mut entrenadores, &mut jugadores, &mut equipos);
}


/*----------INICIO----------*/
	/*
	Al entrar al programa aparece el menú:
	ATLAS RUSO
	1. INICIAR
	2. SALIR
	CAPTURA UNA OPCION: _
	Si no es ninguno imprime opción inválida.
	*/
fn inicio(entrenadores: &mut Vec<Trainer>, jugadores: &mut Vec<Player>, equipos: &mut Vec<Team>) {
    loop {
        print!("\n{}\n{}\n{}\n\n{}\n:", "ATLAS RUSO".bold().yellow().on_red(), "1. INICIAR".green(), "2. SALIR".cyan(), "CAPTURA UNA OPCIÓN".bright_white());
        let mut opt: u8 = 0;
        match scanf!("{u8}", opt) {
            Ok(num) => num,
            Err(_) => {
                println!("{}", INVALID.red());
                continue;
            }
        };

        match opt {
            1 => {
                login(entrenadores, jugadores, equipos);
                continue;
            },

            2 => {
                println!("{}", "\nFin del programa".bold().cyan());
                break;
            },
            _ => {
                println!("{}", INVALID.red());
                continue;
            }
                
        }
    }
    
}

fn login(entrenadores: &mut Vec<Trainer>, jugadores: &mut Vec<Player>, equipos: &mut Vec<Team>) {
    let j:u8 = 0;
    while j == 0 {
        let mut usr = String::new();
        let mut pwrd = String::new();
        print!("\nInserte su nombre de usuario\n:");

        io::stdout().flush().expect("\nflush fallido");

        match io::stdin().read_line(&mut usr) {
            Ok(str) => str,
            Err(_) => {
                println!("{}", "\nUsuario no leido".red());
                continue;
            }
        };
    
        print!("\nInserte su contraseña\n:");

        io::stdout().flush().expect("\nflush fallido");

        match io::stdin().read_line(&mut pwrd) {
            Ok(str) => str,
            Err(_) => {
                println!("{}", "\nUsuario no leido".red());
                continue;
            }
        };
    
        for i in 0..entrenadores.len() {
            let usuario: &str = &usr.trim();
            let contra: &str = &pwrd.trim();
            if usuario == entrenadores[i].TRusrname {
                if contra == entrenadores[i].TRpassword {
                    if i == 0 {
                        menu_admin(entrenadores, jugadores, equipos);
                        let j = 1;
                        break;
                    } else {
                        println!("\nImagina que te envié a menu_usuario"); //Reemplazar por 'menu_usuario'
                        break;
                    } 
                } else {
                    println!("{}", "\nClave incorrecta".red());
                }
            } else {
                println!("{}", "\nUsuario no reconocido".red());
            }
        }
    }

}

/*----------MENU-ADMIN----------*/
	/*
	Al entrar, el ADMIN podrá ver el siguiente menú:
	ATLAS RUSO-ADMIN
	1. ENTRENADORES
	2. REPORTES
	3. CAMBIAR USUARIO
	SELECCIONA UNA OPCION: _
	*/
fn menu_admin(entrenadores: &mut Vec<Trainer>, jugadores: &mut Vec<Player>, equipos: &mut Vec<Team>) {
    loop {
        print!("\n{}\n1. ENTRENADORES\n2. REPORTES\n3. CAMBIAR USUARIO\n4. SALIR\nSELECCIONA UNA OPCIÓN\n:", "ATLAS RUSO-ADMIN".bold());
        let mut opc: u8 = 0;
        match scanf!("{u8}", opc) {
            Ok(num) => num,
            Err(_) => {
                println!("{}", INVALID.red());
                continue;
            }
        };
    
        match opc {
            1 => entrenadores_menu(entrenadores, jugadores, equipos),
            4 => break,
            _ => println!("{}", INVALID.red())
        }     
    }

}

/*-----ENTRENADORES-MENU-----*/
	/*
	Al entrar a entrenadores verá el siguiente menú:
	ENTRENADORES-ADMIN
	1. REGISTRO ENTRENADOR
	2. MOSTRAR ENTRENADORES
	3. BUSCAR ENTRENADOR
	4. EQUIPOS POR ENTRENADOR
	5. PAGOS POR JUGADOR
	6. PAGOS POR FRCHA
	7. REGISTRAR TORNEO
	8. REGRESAR
	SELECCIONA UNA OPCION: _
	*/
fn entrenadores_menu(entrenadores: &mut Vec<Trainer>, jugadores: &mut Vec<Player>, equipos: &mut Vec<Team>) {
    loop {
        print!("\n{}\n1. REGISTRO ENTRENADOR\n2. MOSTRAR ENTRENADORES\n3. BUSCAR ENTRENADOR\n4. EQUIPOS POR ENTRENADOR\n5. PAGOS POR JUGADOR\n6. PAGOS POR FECHA\n7. REGISTRAR TORNEO\n8. REGRESAR\nSELECCIONA UNA OPCIÓN\n:", "ENTRENADORES-ADMIN".bold());
        let mut opcion: u8 = 0;
        match scanf!("{u8}", opcion) {
            Ok(num) => num,
            Err(_) => {
                println!("{}", INVALID.red());
                continue;
            }
        };

        match opcion {
            1 => {
                if entrenadores.len() != ABUELO {
                    entrenadores.push(signup(entrenadores));
                } else {
                    println!("\nLimite de usuarios alcanzado");
                }
                continue
            },
            2 => mostrar(entrenadores),
            
            8 => break,
            _ => println!("{}", INVALID.red())
        };
    }
}

/*-SIGNUP-*/
	/*
	Para REGISTRO, se pedirán los siguientes datos: CODIGO, NOMBRE, AP, AM,
	CARRERA, SUELDO, FECHA INGRESO, regresa al menú de ENTRENADORES. Además,
	se registra un usuario y contraseña (será automáticamente con los datos registrados), el
	usuario será compuesto por las dos primeras iniciales de nombre, ap, am y carrera más un
	número, para diferenciar a los iguales, la contraseña será igual al usuario (solo al momento
	de la creación. Después se cambiará
	*/
fn signup(entrenadores: &Vec<Trainer>) -> Trainer {
    loop {
        print!("\nInserte su código\n:");
        let mut codiguito = 0;
        match scanf!("{}", codiguito) {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "\nCódigo no leido".red());
                continue;
            }
        };
        
        let codiguito = codiguito.to_string();

        for i in 0..entrenadores.len() {
            if entrenadores[i].TRcode.to_string() == codiguito {
                if i == 0{
                    println!("{}", "\nEse usuario ya existe y es el admin".red());
                } else {
                println!("{}", "\nEse usuario ya existe".red());
                }
            } else {
                if i == entrenadores.len() - 1 {
                    let (mut usuarito, mut contraseñita, mut nombresito, mut apsito, mut amsito, mut carrerita, mut dia_reg, usr_num)
                    =   (String::new()       , String::new()           , String::new()         , String::new()     , String::new()     , String::new()        , String::new()      , String::new());

                    print!("\nInserte su Nombre\n:");
                    io::stdout().flush().expect("\nflush fallido");
                    io::stdin().read_line(&mut nombresito).expect(&"Nombre no leido".red());

                    nombresito = nombresito.trim().to_string();
        
                    print!("\nInserte su Apellido Paterno\n:");
                    io::stdout().flush().expect("\nflush fallido");
                    io::stdin().read_line(&mut apsito).expect(&"Apellido no leido".red());

                    apsito = apsito.trim().to_string();
        
                    print!("\nInserte su Apellido Materno\n:");
                    io::stdout().flush().expect("\nflush fallido");
                    io::stdin().read_line(&mut amsito).expect(&"Apellido no leido".red());
        
                    amsito = amsito.trim().to_string();

                    print!("\nInserte su Carrera\n:");
                    io::stdout().flush().expect("\nflush fallido");
                    io::stdin().read_line(&mut carrerita).expect(&"Carrera no leida".red());

                    carrerita = carrerita.trim().to_string();
        
                    print!("\nInserte su Salario\n:");
                    io::stdout().flush().expect("\nflush fallido");
                    let mut salarito = 0;
                    match scanf!("{}", salarito) {
                        Ok(num) => num,
                        Err(_) => {
                            println!("{}", "\nSalario no leido".red());
                            continue;
                        }
                    };
        
                    let salarito = salarito.to_string();
        
                    print!("\nInserte su Día de Registro\n:");
                    io::stdout().flush().expect("\nflush fallido");
                    io::stdin().read_line(&mut dia_reg).expect(&"Dia no leido".red());

                    dia_reg = dia_reg.trim().to_string();
                    
                    if nombresito.len() < 2 {
                        usuarito.push_str(&nombresito[0..nombresito.len()]);
                    } else {
                        usuarito.push_str(&nombresito[0..2]);
                    }
                    
                    if apsito.len() < 2 {
                        usuarito.push_str(&apsito[0..apsito.len()]);
                    } else {
                        usuarito.push_str(&apsito[0..2]);
                    }
                    
                    if amsito.len() < 2 {
                        usuarito.push_str(&amsito[0..amsito.len()]);
                    } else {
                        usuarito.push_str(&amsito[0..2]);
                    }

                    if carrerita.len() < 2 {
                        usuarito.push_str(&carrerita[0..carrerita.len()]);
                    } else {
                        usuarito.push_str(&carrerita[0..2]);
                    }
                    usuarito.push_str(&rand::thread_rng().gen_range(1000..=9999).to_string());
                    usuarito = usuarito.to_lowercase();
        
                    contraseñita.push_str(&usuarito);
                    loop {
                        print!("\nSe han obtenido los siguientes datos:\nCódigo: {}\nNombre: {}\nApellido Paterno: {}\nApellido Materno: {}\nCarrera: {}\nSalario: ${}\nDia de Registro: {}\n\nSu Usuario será: {}\nSu Contraseña será: {}\n\nDesea continuar?\n1. Si\n2. No\n:"
                        ,codiguito.blue(),nombresito.blue(),apsito.blue(),amsito.blue(),  carrerita.blue(), salarito.blue(),dia_reg.blue(),usuarito.bright_black(), contraseñita.bright_black());
                        io::stdout().flush().expect("\nflush fallido");
                        let mut opt:u8 = 0;
                        match scanf!("{}", opt) {
                            Ok(num) => num,
                            Err(_) => {
                                println!("{}", INVALID.red());
                                continue;
                            }
                        };

                        match opt {
                        1 =>
                        return Trainer { TRusrname: usuarito, TRpassword: contraseñita, TRcode: codiguito.to_string(), TRname: nombresito, TRlastName1: apsito, TRlastName2: amsito, TRcareer: carrerita, TRsalary: salarito, TRregDate: dia_reg }
                        ,
                        2 => break,
                        _ => {
                            println!("{}", &INVALID.red());
                            continue
                        }
                        }
                    }

                }
            } 
        }
    }
}
/*-SIGNUP-*/

fn mostrar(entrenadores: &Vec<Trainer>) {
    if entrenadores.len() == 1 {
        println!("\nNo hay usuarios por mostrar");
    } else {
        for i in 1..entrenadores.len() {
            println!("\nCodigo: {}\nNombre: {}\nApellido Paterno: {}\nApellido Materno: {}", entrenadores[i].TRcode.green(), entrenadores[i].TRname.green(), entrenadores[i].TRlastName1.green(), entrenadores[i].TRlastName2.green());
        }
    }
}