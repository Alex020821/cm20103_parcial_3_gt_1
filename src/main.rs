/*
Autor: Alexander Contreras

======================================================
SISTEMA DE CONTROL AÉREO AVL EN RUST
======================================================

FASE 1:
- Explicación de ownership y take()
- Balanceo AVL
- Rotaciones simples y dobles

FASE 2:
- Búsqueda AVL usando referencias de solo lectura

FASE 3:
- Eliminación AVL usando predecesor in-order
- Rebalanceo automático

FASE 4:
- Conteo de vuelos dentro de un rango de altitud
======================================================
*/

#[derive(Debug, Clone)]
struct Vuelo {
    id: String,
    altitud: u32,
}

struct Nodo {
    vuelo: Vuelo,
    izquierdo: Option<Box<Nodo>>,
    derecho: Option<Box<Nodo>>,
    altura: i32,
}

impl Nodo {
    fn nuevo(vuelo: Vuelo) -> Self {
        Nodo {
            vuelo,
            izquierdo: None,
            derecho: None,
            altura: 1,
        }
    }
}

// ======================================================
// FASE 1
// UTILIDADES AVL
// ======================================================

fn obtener_altura(nodo: &Option<Box<Nodo>>) -> i32 {
    nodo.as_ref().map_or(0, |n| n.altura)
}

fn actualizar_altura(nodo: &mut Nodo) {
    nodo.altura = 1 + std::cmp::max(
        obtener_altura(&nodo.izquierdo),
        obtener_altura(&nodo.derecho),
    );
}

fn obtener_balance(nodo: &Nodo) -> i32 {
    obtener_altura(&nodo.izquierdo)
        - obtener_altura(&nodo.derecho)
}

/*
take() mueve la propiedad del nodo hijo
y deja None en el Option original.

Esto evita múltiples propietarios
y garantiza seguridad de memoria.
*/

fn rotar_derecha(mut y: Box<Nodo>) -> Box<Nodo> {

    let mut x = y
        .izquierdo
        .take()
        .expect("Error de radar");

    y.izquierdo = x.derecho.take();

    actualizar_altura(&mut y);

    x.derecho = Some(y);

    actualizar_altura(&mut x);

    x
}

fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {

    let mut y = x
        .derecho
        .take()
        .expect("Error de radar");

    x.derecho = y.izquierdo.take();

    actualizar_altura(&mut x);

    y.izquierdo = Some(x);

    actualizar_altura(&mut y);

    y
}

// ======================================================
// INSERCIÓN AVL
// ======================================================

fn insertar(
    nodo_opt: Option<Box<Nodo>>,
    vuelo: Vuelo
) -> Box<Nodo> {

    let mut nodo = match nodo_opt {
        None => return Box::new(Nodo::nuevo(vuelo)),
        Some(n) => n,
    };

    // Guardamos la altitud antes de mover vuelo
    let altitud = vuelo.altitud;

    if altitud < nodo.vuelo.altitud {

        nodo.izquierdo = Some(
            insertar(
                nodo.izquierdo.take(),
                vuelo
            )
        );

    } else if altitud > nodo.vuelo.altitud {

        nodo.derecho = Some(
            insertar(
                nodo.derecho.take(),
                vuelo
            )
        );

    } else {
        return nodo;
    }

    actualizar_altura(&mut nodo);

    let balance = obtener_balance(&nodo);

    // Caso LL
    if balance > 1 &&
        altitud <
        nodo
            .izquierdo
            .as_ref()
            .unwrap()
            .vuelo
            .altitud {

        return rotar_derecha(nodo);
    }

    // Caso RR
    if balance < -1 &&
        altitud >
        nodo
            .derecho
            .as_ref()
            .unwrap()
            .vuelo
            .altitud {

        return rotar_izquierda(nodo);
    }

    // Caso LR
    if balance > 1 &&
        altitud >
        nodo
            .izquierdo
            .as_ref()
            .unwrap()
            .vuelo
            .altitud {

        let hijo_izq =
            nodo.izquierdo.take().unwrap();

        nodo.izquierdo =
            Some(rotar_izquierda(hijo_izq));

        return rotar_derecha(nodo);
    }

    // Caso RL
    if balance < -1 &&
        altitud <
        nodo
            .derecho
            .as_ref()
            .unwrap()
            .vuelo
            .altitud {

        let hijo_der =
            nodo.derecho.take().unwrap();

        nodo.derecho =
            Some(rotar_derecha(hijo_der));

        return rotar_izquierda(nodo);
    }

    nodo
}

// ======================================================
// FASE 2
// BÚSQUEDA AVL
// ======================================================

fn buscar_vuelo(
    nodo: &Option<Box<Nodo>>,
    altitud: u32
) -> Option<&Vuelo> {

    match nodo {

        None => None,

        Some(n) => {

            if altitud == n.vuelo.altitud {

                Some(&n.vuelo)

            } else if altitud < n.vuelo.altitud {

                buscar_vuelo(
                    &n.izquierdo,
                    altitud
                )

            } else {

                buscar_vuelo(
                    &n.derecho,
                    altitud
                )
            }
        }
    }
}

// ======================================================
// FASE 3
// ELIMINACIÓN AVL
// ======================================================

// Obtener el mayor del subárbol izquierdo
// (predecesor in-order)

fn obtener_maximo(nodo: &Box<Nodo>) -> Vuelo {

    let mut actual = nodo;

    while let Some(ref der) = actual.derecho {
        actual = der;
    }

    actual.vuelo.clone()
}

fn eliminar_vuelo(
    nodo_opt: Option<Box<Nodo>>,
    altitud: u32
) -> Option<Box<Nodo>> {

    let mut nodo = match nodo_opt {
        None => return None,
        Some(n) => n,
    };

    if altitud < nodo.vuelo.altitud {

        nodo.izquierdo = eliminar_vuelo(
            nodo.izquierdo.take(),
            altitud
        );

    } else if altitud > nodo.vuelo.altitud {

        nodo.derecho = eliminar_vuelo(
            nodo.derecho.take(),
            altitud
        );

    } else {

        // Caso 1: sin hijo izquierdo

        if nodo.izquierdo.is_none() {
            return nodo.derecho;
        }

        // Caso 2: sin hijo derecho

        if nodo.derecho.is_none() {
            return nodo.izquierdo;
        }

        // Caso 3: dos hijos

        let predecesor =
            obtener_maximo(
                nodo.izquierdo.as_ref().unwrap()
            );

        nodo.vuelo = predecesor.clone();

        nodo.izquierdo = eliminar_vuelo(
            nodo.izquierdo.take(),
            predecesor.altitud
        );
    }

    actualizar_altura(&mut nodo);

    let balance = obtener_balance(&nodo);

    // LL

    if balance > 1 &&
        obtener_balance(
            nodo.izquierdo.as_ref().unwrap()
        ) >= 0 {

        return Some(rotar_derecha(nodo));
    }

    // LR

    if balance > 1 &&
        obtener_balance(
            nodo.izquierdo.as_ref().unwrap()
        ) < 0 {

        let hijo_izq =
            nodo.izquierdo.take().unwrap();

        nodo.izquierdo =
            Some(rotar_izquierda(hijo_izq));

        return Some(rotar_derecha(nodo));
    }

    // RR

    if balance < -1 &&
        obtener_balance(
            nodo.derecho.as_ref().unwrap()
        ) <= 0 {

        return Some(rotar_izquierda(nodo));
    }

    // RL

    if balance < -1 &&
        obtener_balance(
            nodo.derecho.as_ref().unwrap()
        ) > 0 {

        let hijo_der =
            nodo.derecho.take().unwrap();

        nodo.derecho =
            Some(rotar_derecha(hijo_der));

        return Some(rotar_izquierda(nodo));
    }

    Some(nodo)
}

// ======================================================
// FASE 4
// ALERTA DE PROXIMIDAD
// ======================================================

fn vuelos_en_rango(
    nodo: &Option<Box<Nodo>>,
    min: u32,
    max: u32
) -> usize {

    match nodo {

        None => 0,

        Some(n) => {

            if n.vuelo.altitud < min {

                vuelos_en_rango(
                    &n.derecho,
                    min,
                    max
                )

            } else if n.vuelo.altitud > max {

                vuelos_en_rango(
                    &n.izquierdo,
                    min,
                    max
                )

            } else {

                1 +
                vuelos_en_rango(
                    &n.izquierdo,
                    min,
                    max
                ) +
                vuelos_en_rango(
                    &n.derecho,
                    min,
                    max
                )
            }
        }
    }
}

// ======================================================
// MAIN
// ======================================================

fn main() {

    let mut radar: Option<Box<Nodo>> = None;

    println!("=================================");
    println!("CONTROL AÉREO AVL EN RUST");
    println!("=================================");

    let datos = vec![
        ("AV123", 5000),
        ("UA456", 3000),
        ("IB101", 2000),
        ("AF999", 4000),
        ("TA222", 3500),
        ("AM777", 6000),
    ];

    println!("\nInsertando vuelos...\n");

    for (id, alt) in datos {

        println!(
            "Insertando vuelo {} a {} pies",
            id,
            alt
        );

        let v = Vuelo {
            id: id.to_string(),
            altitud: alt,
        };

        radar = Some(
            insertar(
                radar.take(),
                v
            )
        );
    }

    // ==================================================
    // BÚSQUEDA
    // ==================================================

    println!("\nBuscando vuelo...\n");

    match buscar_vuelo(&radar, 3500) {

        Some(vuelo) => {

            println!("Vuelo encontrado:");

            println!("ID: {}", vuelo.id);

            println!(
                "Altitud: {}",
                vuelo.altitud
            );
        }

        None => {
            println!("Vuelo no encontrado");
        }
    }

    // ==================================================
    // ELIMINACIÓN
    // ==================================================

    println!("\nEliminando vuelo...\n");

    radar = eliminar_vuelo(
        radar.take(),
        3000
    );

    println!(
        "Vuelo con altitud 3000 eliminado"
    );

    // ==================================================
    // ALERTA DE PROXIMIDAD
    // ==================================================

    println!("\nAnalizando proximidad...\n");

    let cantidad = vuelos_en_rango(
        &radar,
        3000,
        5000
    );

    println!(
        "Cantidad de vuelos entre 3000 y 5000 pies: {}",
        cantidad
    );

    println!("\nSistema finalizado.");
}