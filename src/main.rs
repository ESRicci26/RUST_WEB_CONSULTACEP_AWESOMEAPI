use axum::{
    extract::{Extension, Query},
    response::Html,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize}; // Added Serialize
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use reqwest::Client;

#[derive(Debug, Deserialize, Serialize)] // Added Serialize
struct Address {
    address: Option<String>,
    district: Option<String>,
    city: Option<String>,
    state: Option<String>,
    lat: Option<String>,
    lng: Option<String>,
    city_ibge: Option<String>,
    ddd: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SearchForm {
    cep: String,
    api: String,
}

struct AppState {
    client: Client,
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let state = Arc::new(Mutex::new(AppState { client }));

    let app = Router::new()
        .route("/", get(root))
        .route("/search", get(search))
        .layer(axum::Extension(state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<&'static str> {
    Html(r#"
        <html>
            <body>
                <h1>Consulta de CEP Pelas APIs</h1>
                <form id="searchForm">
                    <label>CEP:</label><input type="text" id="cep" required><br>
                    <label>API:</label>
                    <select id="api">
                        <option value="API-AWESOMEAPI">API-AWESOMEAPI</option>
                    </select><br>
                    <button type="button" onclick="fetchData()">Pesquisar</button>
                </form>
                <div id="result">
                    <h2>Resultado:</h2>
                    <form>
                        <label>Endereço:</label><input type="text" id="address" readonly><br>
                        <label>Bairro:</label><input type="text" id="district" readonly><br>
                        <label>Cidade:</label><input type="text" id="city" readonly><br>
                        <label>Estado:</label><input type="text" id="state" readonly><br>
                        <label>Latitude:</label><input type="text" id="lat" readonly><br>
                        <label>Longitude:</label><input type="text" id="lng" readonly><br>
                        <label>Código IBGE:</label><input type="text" id="city_ibge" readonly><br>
                        <label>DDD:</label><input type="text" id="ddd" readonly><br>
                    </form>
                </div>
                <script>
                    async function fetchData() {
                        const cep = document.getElementById('cep').value;
                        const api = document.getElementById('api').value;
                        const response = await fetch(`/search?cep=${cep}&api=${api}`);
                        if (response.ok) {
                            const data = await response.json();
                            document.getElementById('address').value = data.address || '';
                            document.getElementById('district').value = data.district || '';
                            document.getElementById('city').value = data.city || '';
                            document.getElementById('state').value = data.state || '';
                            document.getElementById('lat').value = data.lat || '';
                            document.getElementById('lng').value = data.lng || '';
                            document.getElementById('city_ibge').value = data.city_ibge || '';
                            document.getElementById('ddd').value = data.ddd || '';
                        } else {
                            alert('Erro ao buscar os dados.');
                        }
                    }
                </script>
            </body>
        </html>
    "#)
}

async fn search(
    Extension(state): Extension<Arc<Mutex<AppState>>>, 
    Query(params): Query<SearchForm>,
) -> Html<String> {
    let cep = params.cep.replace(|c: char| !c.is_numeric(), "");
    
    if cep.len() != 8 {
        return Html("CEP inválido".to_string());
    }

    let api_url = match params.api.as_str() {
        "API-AWESOMEAPI" => format!("https://cep.awesomeapi.com.br/json/{}", cep),
        _ => return Html("API não reconhecida.".to_string()),
    };

    let client = &state.lock().await.client;

    match client.get(&api_url).send().await {
        Ok(response) => {
            if let Ok(address) = response.json::<Address>().await {
                return Html(serde_json::to_string(&address).unwrap_or_else(|_| "{}".to_string()));
            } else {
                return Html("Erro ao processar a resposta da API.".to_string());
            }
        }
        Err(_) => Html("Erro ao realizar a consulta.".to_string()),
    }
}
