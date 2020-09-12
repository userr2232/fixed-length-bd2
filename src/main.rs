use serde::{ Serialize, Deserialize };
use fixed_width::{ Reader, FixedWidth, Field, LineBreak };
use std::result::Result;
use std::path::Path;
use fixed_width::Writer;
use std::fs::{ File, OpenOptions };
use std::io::{ prelude::*, BufWriter, BufReader, Seek, SeekFrom };

#[derive(Serialize, Deserialize)]
struct Alumno {
    pub codigo: String,
    pub nombre: String,
    pub apellidos: String,
    pub carrera: String,
}

impl FixedWidth for Alumno {
    fn fields () -> Vec<Field> {
        vec! [
            Field::default().range(0..5),
            Field::default().range(5..17),
            Field::default().range(17..37),
            Field::default().range(37..52),
        ]
    }
}

#[derive(Serialize, Deserialize)]
struct Matricula {
    pub codigo: String,
    pub ciclo: u8,
    pub mensualidad: f32
}

impl FixedWidth for Matricula {
    fn fields () -> Vec<Field> {
        vec![
            Field::default().range(0..5),
            Field::default().range(5..7),
            Field::default().range(7..17)
        ]
    }
}

fn loadAlumnos(path: &str) -> Vec<Alumno> {
    let path = Path::new(&path);
    let mut reader = Reader::from_file(path)
                                .expect("Error while reading from file.")
                                .width(53);
    return reader.byte_reader()
                    .filter_map(Result::ok)
                    .map(|bytes| fixed_width::from_bytes(&bytes).unwrap())
                    .collect();
}

fn loadMatriculas(path: &str) -> Vec<Matricula> {
    let path = Path::new(&path);
    let mut reader = Reader::from_file(path)
                                .expect("Error while reading from file.")
                                .width(18);
    return reader.byte_reader()
                    .filter_map(Result::ok)
                    .map(|bytes| fixed_width::from_bytes(&bytes).unwrap())
                    .collect();
}

fn add<T: FixedWidth + Serialize>(record: T, path: &str) {
    let mut file = File::open(path).expect("Error while reading from file.");
    let reader = BufReader::new(file);
    let mut bytes: u64 = 0;
    let lines = reader.lines();
    for line in lines {
        let line = line.unwrap();
        if line.chars().next().unwrap() != ' ' {
            let mut file = OpenOptions::new()
                                .append(true)
                                .create(true)
                                .open(path)
                                .unwrap();
            let writer = BufWriter::new(file);
            let mut writer = Writer::from_writer(writer)
                                        .linebreak(LineBreak::Newline);
            writer.write_serialized(vec![record]
                    .into_iter())
                    .unwrap();
            writer.write_linebreak()
                    .unwrap();
            break;
        }
    }
}

fn delete(pos: u64, path: &str) {
    let mut file = File::open(path).expect("Error while reading from file.");
    let reader = BufReader::new(file);
    let mut lineN: u64 = 0;
    let mut bytes: u64 = 0;
    let lines = reader.lines();
    for line in lines {
        let line = line.unwrap();
        if line.chars().next().unwrap() != ' ' {
            lineN = lineN + 1;
            if lineN == pos {
                let mut file = OpenOptions::new()
                                            .write(true)
                                            .create(true)
                                            .open(path)
                                            .unwrap();
                file.seek(SeekFrom::Start(bytes))
                    .unwrap();
                file.write_all(" ".repeat(line.bytes().len()).as_bytes())
                    .unwrap();
                file.write(b"\n");
                break;
            }
        }
        bytes = bytes + 1 + line.bytes().len() as u64;
    }
}

fn main() {
    {
        let alumnoIn = "static/alumnos/in.txt";
        let alumnoOut = "static/alumnos/out.txt";

        let alumnos: Vec<Alumno> = loadAlumnos(alumnoIn);
        for alumno in &alumnos {
            println!("-{}-{}-{}-{}-", (*alumno).codigo, (*alumno).nombre, (*alumno).apellidos, (*alumno).carrera);
        }
        let alumno = Alumno {
            codigo: "0005".to_string(),
            nombre: "Nombre".to_string(),
            apellidos: "Apellido X".to_string(),
            carrera: "Carrera".to_string()
        };
        add(alumno, alumnoOut);
        delete(24, alumnoOut);
    }
    
    {
        let matriculaIn = "static/matriculas/in.txt";
        let matriculaOut = "static/matriculas/out.txt";

        let matriculas: Vec<Matricula> = loadMatriculas(&matriculaIn);
        for matricula in &matriculas {
            println!("-{}-{}-{}-", (*matricula).codigo, (*matricula).ciclo, (*matricula).mensualidad);
        }
        let matricula = Matricula {
            codigo: "0001".to_string(),
            ciclo: 5,
            mensualidad: 2000.50
        };
        add(matricula, matriculaOut);
        // delete(3, matriculaOut);
    }
}