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

use std::{io::{self, Write}, vec, fmt::*, process::exit, f32::consts::E};
use colored::*;
use core::mem::size_of;
use rand::prelude::*;
use scanf::scanf;

const LIM: u8 = 30;
const LIM2: u8 = 60;
const ABUELO: usize = 255;
const INVALID: &str = "\nOpción inválida";
const NTN: &str = "\nNo se ingresó un número válido";
const NTR: &str = "\nOpción no leida";
const NRS: &str = "\nNo se ha encontrado ningun resultado.";

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

/*----------PRINCIPAL----------*/
fn main() {
    let mut equipos: Vec<Team> = Vec::with_capacity(ABUELO);
    let mut entrenadores: Vec<Trainer> = Vec::with_capacity(ABUELO);
    let mut jugadores: Vec<Player> = Vec::with_capacity(ABUELO);
    let mut pagos: Vec<Check> = Vec::with_capacity(ABUELO);

    entrenadores.push(Trainer::to_admin());

    inicio(&mut entrenadores, &mut jugadores, &mut equipos, &mut pagos);
}
/*----------/PRINCIPAL----------*/
/*----------INICIO----------*/
	/*
	Al entrar al programa aparece el menú:
	ATLAS RUSO
	1. INICIAR
	2. SALIR
	CAPTURA UNA OPCION: _
	Si no es ninguno imprime opción inválida.
	*/
fn inicio(entrenadores: &mut Vec<Trainer>, jugadores: &mut Vec<Player>, equipos: &mut Vec<Team>, pagos: &mut Vec<Check>) {
    loop {
        print!("\n{}\n{}\n{}\n\n{}\n:", "ATLAS RUSO".bold().yellow().on_red(), "1. INICIAR".green(), "2. SALIR".cyan(), "CAPTURA UNA OPCIÓN.".bright_white());
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
                login(entrenadores, jugadores, equipos, pagos);
                continue;
            },
            2 => {
                println!("{}", "\nFin del programa.".bold().cyan());
                break;
            },
            _ => {
                println!("{}", INVALID.red());
                continue;
            }
                
        }
    }
}
/*----------/INICIO----------*/
/*----------LOGIN----------*/
fn login(entrenadores: &mut Vec<Trainer>, jugadores: &mut Vec<Player>, equipos: &mut Vec<Team>, pagos: &mut Vec<Check>) {
    let mut j:bool = false;
    let mut times = 0;
    while !j {
        let mut usr = String::new();
        let mut pwrd = String::new();
        print!("{}", "\nInserte su nombre de usuario.\n:".bright_white());
        match io::stdout().flush() {
            Ok(fl) => fl,
            Err(_) => {
                println!("{}", "\nFlush fallido.".red());
                continue;
            }
        }
        match io::stdin().read_line(&mut usr) {
            Ok(str) => str,
            Err(_) => {
                println!("{}", "\nUsuario no leido.".red());
                continue;
            }
        };
        print!("{}", "\nInserte su contraseña.\n:".bright_white());
        match io::stdout().flush() {
            Ok(fl) => fl,
            Err(_) => {
                println!("{}", "\nFlush fallido.".red());
                continue;
            }
        }
        match io::stdin().read_line(&mut pwrd) {
            Ok(str) => str,
            Err(_) => {
                println!("{}", "\nUsuario no leido.".red());
                continue;
            }
        };
        for i in 0..entrenadores.len() {
            times += 1;
            let usuario: &str = &usr.trim();
            let contra: &str = &pwrd.trim();
            if usuario == entrenadores[i].TRusrname {
                if contra == entrenadores[i].TRpassword {
                    if i == 0 {
                        j = menu_admin(entrenadores, jugadores, equipos, pagos);
                        break;
                    } else {
                        println!("\nImagina que te envié a menu_usuario."); //Reemplazar por 'menu_usuario'
                        j = true;
                        break;
                    } 
                } else {
                    println!("{}", "\nClave incorrecta.".red());
                }
            } 
        }
        if times == entrenadores.len() {
            println!("{}", "\nUsuario no reconocido.".red());
        }
    }
}
/*----------/LOGIN----------*/
/*----------MENU-ADMIN----------*/
	/*
	Al entrar, el ADMIN podrá ver el siguiente menú:
	ATLAS RUSO-ADMIN
	1. ENTRENADORES
	2. REPORTES
	3. CAMBIAR USUARIO
	SELECCIONA UNA OPCION: _
	*/
fn menu_admin(entrenadores: &mut Vec<Trainer>, jugadores: &mut Vec<Player>, equipos: &mut Vec<Team>, pagos: &mut Vec<Check>) -> bool {
    loop {
        print!("\n{}\n1. ENTRENADORES\n2. REPORTES\n3. CAMBIAR USUARIO\n4. SALIR\n{}\n:", "ATLAS RUSO-ADMIN".bold(), "SELECCIONA UNA OPCIÓN.".bright_white());
        let mut opc: u8 = 0;
        match scanf!("{u8}", opc) {
            Ok(num) => num,
            Err(_) => {
                println!("{}", INVALID.red());
                continue;
            }
        };
        match opc {
            1 => entrenadores_menu(entrenadores, jugadores, equipos, pagos),
            3 => return false,
            4 => return true,
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
fn entrenadores_menu(entrenadores: &mut Vec<Trainer>, jugadores: &mut Vec<Player>, equipos: &mut Vec<Team>, pagos: &mut Vec<Check>) {
    loop {
        print!("\n{}{}{}\n:", "ENTRENADORES-ADMIN".bold(), "\n1. REGISTRO ENTRENADOR\n2. MOSTRAR ENTRENADORES\n3. BUSCAR ENTRENADOR\n4. EQUIPOS POR ENTRENADOR\n5. PAGOS POR JUGADOR\n6. PAGOS POR FECHA\n7. REGRESAR\n".yellow(), "SELECCIONA UNA OPCIÓN.".bright_white());
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
                if entrenadores.len() < ABUELO {
                    entrenadores.push(signup(entrenadores));
                } else {
                    println!("{}", "\nLímite de usuarios alcanzado.".red());
                }
            },
            2 => mostrar(entrenadores),
            3 => buscar(entrenadores),
            4 => equipoXCoach(entrenadores, equipos),
            5 => pagosJugador(jugadores, pagos),
            6 => pagosFecha(jugadores, pagos),
            7 => break,
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
        print!("{}", "\nInserte su Código.\n:".bright_white());
        let mut codiguito = 0;
        match scanf!("{}", codiguito) {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "\nCódigo no leido.".red());
                continue;
            }
        };
        let codiguito = codiguito.to_string();
        for i in 0..entrenadores.len() {
            if entrenadores[i].TRcode.to_string() == codiguito {
                if i == 0{
                    println!("{}", "\nEse usuario ya existe y es el admin.".red());
                } else {
                println!("{}", "\nEse usuario ya existe.".red());
                }
            } else {
                if i == entrenadores.len() - 1 {
                    let (mut usuarito, mut contraseñita, mut nombresito, mut apsito, mut amsito, mut carrerita, mut dia_reg, usr_num)
                    =   (String::new()       , String::new()           , String::new()         , String::new()     , String::new()     , String::new()        , String::new()      , String::new());
                    print!("\nInserte su Nombre.\n:");
                    match io::stdout().flush() {
                        Ok(fl) => fl,
                        Err(_) => {
                            println!("{}", "\nFlush fallido.".red());
                            continue;
                        }
                    }
                    match io::stdin().read_line(&mut nombresito) {
                        Ok(str) => str,
                        Err(_) => {
                            print!("{}", "Nombre no leido.".red());
                            continue;
                        }
                    };
                    nombresito = nombresito.trim().to_string();
                    print!("\nInserte su Apellido Paterno.\n:");
                    match io::stdout().flush() {
                        Ok(fl) => fl,
                        Err(_) => {
                            println!("{}", "\nFlush fallido.".red());
                            continue;
                        }
                    }
                    match io::stdin().read_line(&mut apsito) {
                        Ok(str) => str,
                        Err(_) => {
                            print!("{}", "Apellido no leido.".red());
                            continue;
                        }
                    };
                    apsito = apsito.trim().to_string();
                    print!("\nInserte su Apellido Materno.\n:");
                    match io::stdout().flush() {
                        Ok(fl) => fl,
                        Err(_) => {
                            println!("{}", "\nFlush fallido.".red());
                            continue;
                        }
                    }
                    match io::stdin().read_line(&mut amsito) {
                        Ok(str) => str,
                        Err(_) => {
                            print!("{}", "Apellido no leido.".red());
                            continue;
                        }
                    };
                    amsito = amsito.trim().to_string();
                    print!("\nInserte su Carrera.\n:");
                    match io::stdout().flush() {
                        Ok(fl) => fl,
                        Err(_) => {
                            println!("{}", "\nFlush fallido.".red());
                            continue;
                        }
                    }
                    match io::stdin().read_line(&mut carrerita) {
                        Ok(str) => str,
                        Err(_) => {
                            print!("{}", "Carrera no leida.".red());
                            continue;
                        }
                    };
                    carrerita = carrerita.trim().to_string();
                    print!("\nInserte su Salario.\n:");
                    match io::stdout().flush() {
                        Ok(fl) => fl,
                        Err(_) => {
                            println!("{}", "\nFlush fallido.".red());
                            continue;
                        }
                    }
                    let mut salarito:f32 = 0.0;
                    match scanf!("{}", salarito) {
                        Ok(num) => num,
                        Err(_) => {
                            println!("{}", "\nSalario no leido.".red());
                            continue;
                        }
                    };
                    let salarito = salarito.to_string();
                    print!("\nInserte su Día de Registro.\n:");
                    match io::stdout().flush() {
                        Ok(fl) => fl,
                        Err(_) => {
                            println!("{}", "\nFlush fallido.".red());
                            continue;
                        }
                    }
                    match io::stdin().read_line(&mut dia_reg) {
                        Ok(str) => str,
                        Err(_) => {
                            print!("{}", "Día no leido.".red());
                            continue;
                        }
                    };
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
                        print!("\nSe han obtenido los siguientes datos:\nCódigo: {}\nNombre: {}\nApellido Paterno: {}\nApellido Materno: {}\nCarrera: {}\nSalario: ${}\nDia de Registro: {}\n\nSu Usuario será: {}\nSu Contraseña será: {}\n\n{}\n1. Si\n2. No\n:"
                        ,codiguito.blue(),nombresito.blue(),apsito.blue(),amsito.blue(),  carrerita.blue(), salarito.blue(),dia_reg.blue(),usuarito.bright_black(), contraseñita.bright_black(), "Desea continuar?.".bright_white());
                        match io::stdout().flush() {
                            Ok(fl) => fl,
                            Err(_) => {
                                println!("{}", "\nFlush fallido.".red());
                                continue;
                            }
                        }
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
/*-MOSTRAR-*/
	/*
	Para MOSTRAR ENTRENADORES, se muestra la lista de entrenadores, código, nombre y
	apellidos, regresa al menú de ENTRENADORES.
	*/
fn mostrar(entrenadores: &Vec<Trainer>) {
    if entrenadores.len() == 1 {
        println!("\nNo hay usuarios por mostrar.");
    } else {
        for i in 1..entrenadores.len() {
            println!("\nCodigo: {}\nNombre: {}\nApellido Paterno: {}\nApellido Materno: {}", entrenadores[i].TRcode.green(), entrenadores[i].TRname.green(), entrenadores[i].TRlastName1.green(), entrenadores[i].TRlastName2.green());
        }
    }
}
/*-/MOSTRAR-*/
/*-BUSCAR-*/
	/*
	Para BUSCAR ENTRENADOR, se busca por código, si no se encuentra imprime mensaje,
	si se encuentra muestra su información, regresa al menú de ENTRENADORES.
	*/
fn buscar(entrenadores: &Vec<Trainer>) {
    let mut j = 0;
    let mut codiguito = String::new();
    if entrenadores.len() == 1 {
        println!("\nNo hay usuarios por buscar.");
    } else {
        print!("\nInserte el código del entrenador a buscar.\n:");
        match io::stdout().flush() {
            Ok(fl) => fl,
            Err(_) => {
                println!("{}", "\nFlush fallido.".red())
            }
        }
        match io::stdin().read_line(&mut codiguito) {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "\nCódigo no leido.".red());
                return;
            }
        };
        codiguito = codiguito.trim().to_string();
        let codiguito = codiguito.to_string();
        for i in 0..entrenadores.len() {
            if entrenadores[i].TRcode == codiguito {
            print!("\nCodigo: {}\nNombre: {}\nApellido Paterno: {}\nApellido Materno: {}\n", entrenadores[i].TRcode.green(), entrenadores[i].TRname.green(), entrenadores[i].TRlastName1.green(), entrenadores[i].TRlastName2.green());
                j = 1;
                break;
            }
        }
        if j == 0 {
            println!("{}", NRS.red());
        }
    }
}
/*-/BUSCAR-*/
/*-EQUIPOXCOACH-*/
	/*
	En EQUIPO POR ENTRENADOR, se pregunta el código del entrenador y muestra cuantos
	y cuales equipos tiene, regresa al menú de ENTRENADORES.
	*/
fn equipoXCoach(entrenadores: &mut Vec<Trainer>, equipos: &mut Vec<Team>) {
    let mut cuenta = 0;
    let mut codiguito = String::new();
    let (mut exists,mut found) = (false, false);
    if entrenadores.len() == 1 {
        println!("\nNo hay usuarios por buscar.");
    } else {
        print!("\nInserte el codigo del entrenador a buscar\n:");
        match io::stdout().flush() {
            Ok(fl) => fl,
            Err(_) => {
                println!("{}", "\nFlush fallido.".red());
                return;
            }
        }
        match io::stdin().read_line(&mut codiguito) {
            Ok(num) => num,
            Err(_) => {
                println!("{}", INVALID.red());
                return;
            }
        };
        codiguito = codiguito.trim().to_string();
        for i in 1..entrenadores.len() {
            if entrenadores[i].TRcode == codiguito {
                exists = true;
                for j in 0..equipos.len() {
                    if equipos[j].EQcoachID == codiguito {
                        found = true;
                        print!("\nCodigo de equipo: {}.", equipos[j].EQteamID);
                        cuenta += 1;
                    }
                }
                if !found {
                    println!("{}", "\nEste entrenador no tiene equipos.".red());
                } else if cuenta == 1 {
                    println!("{}{}{}", "\nEste entrenador tiene".green(), "1".green(), "equipo.".green());
                } else {
                    println!("{}{}{}", "\nEste entrenador tiene".green(), cuenta.to_string().green(), "equipos.".green());
                }
                break;
            }
        }
        if !exists {
            println!("{}", NRS.red());
        }
    }
}
/*-/EQUIPOXCOACH-*/
/*-PAGOSJUGADOR-*/
	/*
	PAGOS POR JUGADOR, pregunta el código del jugador, si no está anda m mensaje, si lo
	encuentra muestra cuantas semanas ha pagado y su equivalente en dinero.
	*/
fn pagosJugador(jugadores: &mut Vec<Player>, pagos: &mut Vec<Check>) {
	let mut codiguito = String::new();
    let mut found = false;
	if jugadores.len() == 0 {
        println!("\nNo hay usuarios por buscar.");
    } else {
        print!("\nInserte el codigo del entrenador a buscar.\n:");
        match io::stdout().flush() {
            Ok(fl) => fl,
            Err(_) => {
                println!("{}", "\nFlush fallido.".red());
                return;
            }
        }
        match io::stdin().read_line(&mut codiguito) {
            Ok(num) => num,
            Err(_) => {
                println!("{}", INVALID.red());
                return;
            }
        };
        codiguito = codiguito.trim().to_string();
        for i in 0..jugadores.len() {
            if pagos[i].CHplayerID == codiguito {
                found = true;
				print!("\nCodigo de entrenador: {}\nCantidad: {}\nFecha: {}\nSemana: {}.", pagos[i].CHcoachID, pagos[i].CHamount, pagos[i].CHdate, pagos[i].CHweek);
            }
        }
        if !found {
            println!("\nNo se ha encontrado ningun resultado.");
        }
    }
}
/*-/PAGOSJUGADOR-*/
/*-PAGOSFECHA-*/
	/*
	PAGOS POR FRCHA, pregunta la fecha, si la fecha no existe o no es válida, mandar
	mensaje, si está muestra cuantos pagos se hicieron y su monto en dinero.
	*/
fn pagosFecha(jugadores: &mut Vec<Player>, pagos: &mut Vec<Check>) {
    let mut fecha = String::new();
    let mut found = false;

    if jugadores.len() == 0 {
        println!("\nNo hay usuarios por buscar.");
    } else {
        print!("\nInserte la fecha del pago a buscar.\n:");
        match io::stdout().flush() {
            Ok(fl) => fl,
            Err(_) => {
                println!("{}", "\nFlush fallido.".red());
                return;
            }
        }
        match io::stdin().read_line(&mut fecha) {
            Ok(num) => num,
            Err(_) => {
                println!("{}", INVALID.red());
                return;
            }
        };
        fecha = fecha.trim().to_string();
        for i in 0..jugadores.len() {
            if pagos[i].CHdate == fecha {
                found = true;
                print!("\nCodigo del jugador: {}\nCodigo de entrenador: {}\nCantidad: {}\nSemana: {}.", pagos[i].CHplayerID, pagos[i].CHcoachID, pagos[i].CHamount, pagos[i].CHweek);
            }
        }
        if !found {
            println!("{}", NRS.red());
        }
    }
}
/*-/PAGOSFECHA-*/
/*-----/ENTRENADORES-MENU-----*/
/*-----REPORTES-----*/
	/*
	Al entrar a REPORTES muestra:
	REPORTES-ADMIN
	1. REGISTRO EQUIPOS
	2. REGISTRO DE JUGADORES
	3. MOSTRAR EQUIPOS
	4. BUSCAR EQUIPO POR CATEGORÍA
	5. BUSCAR EQUIPO POR CÓDIGO
	6. MOSTRAR TORNEOS
	7. REGRESAR
	SELECCIONA UNA OPCIÓN: _
	*/
fn rep_admin(entrenadores: &mut Vec<Trainer>, jugadores: &mut Vec<Player>, equipos: &mut Vec<Team>, pagos: &mut Vec<Check>) {
    loop {
        print!("\n{}{}{}\n:", "REPORTES-ADMIN".bold(), "\n1. REGISTRO EQUIPOS\n2. REGISTRO JUGADORES\n3. MOSTRAR EQUIPOS\n4. BUSCAR EQUIPO POR CATEGORÍA\n5. BUSCAR EQUIPO POR CÓDIGO\n6. MOSTRAR TORNEOS\n7. REGRESAR\n".yellow(), "SELECCIONA UNA OPCIÓN.".bright_white());
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
                if entrenadores.len() < ABUELO {
                    entrenadores.push(signup(entrenadores));
                } else {
                    println!("{}", "\nLímite de usuarios alcanzado.".red());
                }
            },
            2 => mostrar(entrenadores),//
            3 => buscar(entrenadores),//
            4 => equipoXCoach(entrenadores, equipos),//
            5 => pagosJugador(jugadores, pagos),//
            6 => pagosFecha(jugadores, pagos),//
            7 => break,
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
fn reg_equip(entrenadores: &Vec<Trainer>) -> Trainer {
    loop {
        print!("{}", "\nInserte el Código del equipo.\n:".bright_white());
        let mut codiguito = 0;
        match scanf!("{}", codiguito) {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "\nCódigo no leido.".red());
                continue;
            }
        };
        let codiguito = codiguito.to_string();
        for i in 0..entrenadores.len() {
            if entrenadores[i].TRcode.to_string() == codiguito {
                if i == 0{
                    println!("{}", "\nEse usuario ya existe y es el admin.".red());
                } else {
                println!("{}", "\nEse usuario ya existe.".red());
                }
            } else {
                if i == entrenadores.len() - 1 {
                    let (mut usuarito, mut contraseñita, mut nombresito, mut apsito, mut amsito, mut carrerita, mut dia_reg, usr_num)
                    =   (String::new()       , String::new()           , String::new()         , String::new()     , String::new()     , String::new()        , String::new()      , String::new());
                    print!("\nInserte su Nombre.\n:");
                    match io::stdout().flush() {
                        Ok(fl) => fl,
                        Err(_) => {
                            println!("{}", "\nFlush fallido.".red());
                            continue;
                        }
                    }
                    match io::stdin().read_line(&mut nombresito) {
                        Ok(str) => str,
                        Err(_) => {
                            print!("{}", "Nombre no leido.".red());
                            continue;
                        }
                    };
                    nombresito = nombresito.trim().to_string();
                    print!("\nInserte su Apellido Paterno.\n:");
                    match io::stdout().flush() {
                        Ok(fl) => fl,
                        Err(_) => {
                            println!("{}", "\nFlush fallido.".red());
                            continue;
                        }
                    }
                    match io::stdin().read_line(&mut apsito) {
                        Ok(str) => str,
                        Err(_) => {
                            print!("{}", "Apellido no leido.".red());
                            continue;
                        }
                    };
                    apsito = apsito.trim().to_string();
                    print!("\nInserte su Apellido Materno.\n:");
                    match io::stdout().flush() {
                        Ok(fl) => fl,
                        Err(_) => {
                            println!("{}", "\nFlush fallido.".red());
                            continue;
                        }
                    }
                    match io::stdin().read_line(&mut amsito) {
                        Ok(str) => str,
                        Err(_) => {
                            print!("{}", "Apellido no leido.".red());
                            continue;
                        }
                    };
                    amsito = amsito.trim().to_string();
                    print!("\nInserte su Carrera.\n:");
                    match io::stdout().flush() {
                        Ok(fl) => fl,
                        Err(_) => {
                            println!("{}", "\nFlush fallido.".red());
                            continue;
                        }
                    }
                    match io::stdin().read_line(&mut carrerita) {
                        Ok(str) => str,
                        Err(_) => {
                            print!("{}", "Carrera no leida.".red());
                            continue;
                        }
                    };
                    carrerita = carrerita.trim().to_string();
                    print!("\nInserte su Salario.\n:");
                    match io::stdout().flush() {
                        Ok(fl) => fl,
                        Err(_) => {
                            println!("{}", "\nFlush fallido.".red());
                            continue;
                        }
                    }
                    let mut salarito:f32 = 0.0;
                    match scanf!("{}", salarito) {
                        Ok(num) => num,
                        Err(_) => {
                            println!("{}", "\nSalario no leido.".red());
                            continue;
                        }
                    };
                    let salarito = salarito.to_string();
                    print!("\nInserte su Día de Registro.\n:");
                    match io::stdout().flush() {
                        Ok(fl) => fl,
                        Err(_) => {
                            println!("{}", "\nFlush fallido.".red());
                            continue;
                        }
                    }
                    match io::stdin().read_line(&mut dia_reg) {
                        Ok(str) => str,
                        Err(_) => {
                            print!("{}", "Día no leido.".red());
                            continue;
                        }
                    };
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
                        print!("\nSe han obtenido los siguientes datos:\nCódigo: {}\nNombre: {}\nApellido Paterno: {}\nApellido Materno: {}\nCarrera: {}\nSalario: ${}\nDia de Registro: {}\n\nSu Usuario será: {}\nSu Contraseña será: {}\n\n{}\n1. Si\n2. No\n:"
                        ,codiguito.blue(),nombresito.blue(),apsito.blue(),amsito.blue(),  carrerita.blue(), salarito.blue(),dia_reg.blue(),usuarito.bright_black(), contraseñita.bright_black(), "Desea continuar?.".bright_white());
                        match io::stdout().flush() {
                            Ok(fl) => fl,
                            Err(_) => {
                                println!("{}", "\nFlush fallido.".red());
                                continue;
                            }
                        }
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