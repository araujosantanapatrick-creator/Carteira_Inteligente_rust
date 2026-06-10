#[derive(Debug)]
struct Ativo {
    nome: String,
    quantidade: f64,
    preco_medio: f64,
    preco_atual: f64,
}

impl Ativo {
    fn novo(
        nome: String,
        quantidade: f64,
        preco_medio: f64,
        preco_atual: f64,
    ) -> Result<Ativo, String> {
        if nome.trim().is_empty() {
            return Err(String::from("Nome do ativo inválido."));
        }

        if quantidade <= 0.0 {
            return Err(String::from("Quantidade deve ser maior que zero."));
        }

        if preco_medio <= 0.0 || preco_atual <= 0.0 {
            return Err(String::from("Preços devem ser maiores que zero."));
        }

        Ok(Ativo {
            nome,
            quantidade,
            preco_medio,
            preco_atual,
        })
    }

    fn valor_investido(&self) -> f64 {
        self.quantidade * self.preco_medio
    }

    fn valor_atual(&self) -> f64 {
        self.quantidade * self.preco_atual
    }

    fn rentabilidade_percentual(&self) -> f64 {
        ((self.valor_atual() - self.valor_investido()) / self.valor_investido()) * 100.0
    }
}

fn main() {
    let ativo = Ativo::novo(
        String::from("ITUB4"),
        200.0,
        30.00,
        34.50,
    );

    match ativo {
        Ok(a) => {
            println!("=== Carteira Inteligente de Investimentos ===");
            println!("Ativo: {}", a.nome);
            println!("Quantidade: {}", a.quantidade);
            println!("Preço Médio: R$ {:.2}", a.preco_medio);
            println!("Preço Atual: R$ {:.2}", a.preco_atual);
            println!("Valor Investido: R$ {:.2}", a.valor_investido());
            println!("Valor Atual: R$ {:.2}", a.valor_atual());
            println!(
                "Rentabilidade: {:.2}%",
                a.rentabilidade_percentual()
            );
        }
        Err(erro) => {
            println!("Erro ao cadastrar ativo: {}", erro);
        }
    }
}