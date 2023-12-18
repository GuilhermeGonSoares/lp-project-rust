# Histórico e Versões da linguagem de Rust

Criada em 2010 pela Mozilla com o propósito de fornecer uma alternativa mais segura e mais rápida ao C++. ideia por trás da linguagem era combinar a eficiência de C++ com a segurança e a prevenção de falhas de linguagens de programação como Java e C#.

A ideia por trás do projeto era criar uma linguagem para programação de sistemas, ou seja, para a construção de softwares de alto desempenho e seguros que pudessem ser usados em áreas como aeroespacial, medicina, finanças e outros setores críticos

Desde o seu lançamento em 2010, Rust tem sido desenvolvida de forma aberta e colaborativa, com várias contribuições da comunidade. A linguagem tem crescido em popularidade nos últimos anos, com empresas como Microsoft, Amazon, Google, Dropbox, Facebook e outras adotando a linguagem para seus projetos.

Aqui está uma lista das principais versões de Rust:

    Rust 1.0 (15 de maio de 2015):
    Lançamento estável inicial.

    Rust 1.18 (29 de junho de 2017):
    Introdução de async e await para suporte a programação assíncrona.

    Rust 1.28 (2 de agosto de 2018):
    Introdução do "const generics", permitindo a parametrização por valores constantes.

    Rust 1.39 (7 de novembro de 2019):
    Adição do operador ? para facilitar o tratamento de erros em funções assíncronas.

    Rust 1.41 (12 de dezembro de 2019):
    Adição do tipo de dados Pin para melhor suporte a programação assíncrona.

    Rust 1.51 (25 de fevereiro de 2021):
    Adição de "const generics" em structs e enums.

    Rust 1.58 (24 de novembro de 2021):
    Adição do operador let em padrões.

# Premissas, Usuário Característico e Domínio de Aplicação do Gerenciador de Senhas com Criptografia

A **Premissa** do nosso projeto de controle financeiro é que o usuário possa armazenar transações de gastos e ganhos classificando o tipo e mantendo um controle sobre saldo, total de gastos e de ganhos. Também oferecemos uma interface de uso simples e fácil.

O **usuário característico** do gerenciador de finanças é uma pessoa ou organização que precisa gerenciar suas despesas e ter maior controle financeiro sobre seus gastos. O usuário característico também pode ter diferentes níveis de conhecimento técnico, desde iniciantes até usuários avançados, as funcionalidades básicas podem ser úteis pra qualquer pessoa.

O **domínio de aplicação** do gerenciador de finanças inclui uma ampla gama de setores, desde indivíduos pequenas empresas. O gerenciador de finanças pode ser usado para gerenciar finanças pessoais mas também pode ser usado por pequenas empresas para gastos com estoque e fornecedores, pagamento de funcionários e os valores de vendas e margem de lucro. Em resumo, o gerenciador de finanças pode ser usado em qualquer situação em que haja a necessidade de gerenciar gastos e ganhos de forma eficiente.

### Construtores

Os **Construtores** em rust, ou melhor o processo de inicialização de um objeto ou criação de instâncias de um objeto/struct é feita por meio de funções associadas, chamadas de New. Para usar as funções associadas declaramos a impl da classe em que incluímos métodos, as funções associadas e até interfaces para a classe.

```rust
struct MinhaEstrutura {
    valor: i32,
}

impl MinhaEstrutura {
    // Função associada para criar uma instância com um valor padrão
    fn novo() -> MinhaEstrutura {
        MinhaEstrutura { valor: 0 }
    }

    // Função associada para criar uma instância com um valor específico
    fn com_valor(valor: i32) -> MinhaEstrutura {
        MinhaEstrutura { valor }
    }
}

fn main() {
    // Criando instâncias usando funções associadas
    let objeto1 = MinhaEstrutura::novo();             // Chama a função associada novo()
    let objeto2 = MinhaEstrutura::com_valor(42);      // Chama a função associada com_valor()

    // Imprimindo os valores
    println!("Objeto 1: valor = {}", objeto1.valor);
    println!("Objeto 2: valor = {}", objeto2.valor);
}
```

Contrastando com a estrutura de instanciação de objetos de C++. Aqui basta declarar um objeto com os argumentos necessários para criar uma nova instância.

```cpp
struct MinhaEstrutura {
    int valor;

    // Construtor padrão
    MinhaEstrutura() : valor(0) {}

    // Construtor com valor específico
    explicit MinhaEstrutura(int val) : valor(val) {}
};

int main() {
    // Criando instâncias
    MinhaEstrutura objeto1;              // Chama o construtor padrão
    MinhaEstrutura objeto2(42);          // Chama o construtor com valor

    // Imprimindo os valores
    std::cout << "Objeto 1: valor = " << objeto1.valor << std::endl;
    std::cout << "Objeto 2: valor = " << objeto2.valor << std::endl;

    return 0;
}
```

# Legibilidade

### Simplicidade Global:

Moderada. Rust possui vários componentes básicos e abstratos, como 'ownership', 'lifetimes', e 'borrowing'. Esses conceitos são fundamentais para garantir a segurança de memória, mas podem adicionar uma camada de complexidade para novos programadores.

### Multiplicidade de Recursos:

Boa. Rust tende a oferecer múltiplas maneiras de realizar uma operação, mas isso é geralmente feito para proporcionar flexibilidade sem comprometer a segurança e a performance. Por exemplo, iteração sobre coleções pode ser feita de várias formas, mas todas são seguras e eficientes.

### Sobrecarga:

Moderada. Rust suporta sobrecarga de operadores e traits, mas de uma maneira controlada que tende a evitar o abuso ou a confusão.

### Ortogonalidade:

Alta. Rust oferece um alto grau de ortogonalidade; muitos de seus conceitos básicos podem ser combinados de maneiras significativas. No entanto, isso pode aumentar a curva de aprendizado.

### Significado Independente do Contexto:

Boa. Rust é uma linguagem com forte tipagem estática, o que significa que o significado de muitas construções é claro e independente do contexto.

### Instruções de Controle:

Excelente. Rust evita instruções de controle problemáticas como goto, favorecendo estruturas de controle mais modernas e seguras.

### Tipos de Dados e Estruturas:

Excelente. Rust oferece uma ampla gama de tipos de dados primitivos e estruturas, com facilidades para defini-los, incluindo tipos booleanos, registros (structs), enumerações, e coleções poderosas.

### Aspectos da Sintaxe:

Boa. Rust usa uma sintaxe relativamente clara com palavras-chave significativas e uma forma que geralmente é compatível com sua semântica.

# Capacidade de Escrita

### Simplicidade e Ortogonalidade:

**Poucos Componentes Básicos e Regras Consistentes:** Rust tem um conjunto de conceitos básicos bem definidos, como propriedade, empréstimo e tempo de vida (lifetimes), que são consistentes em toda a linguagem. No entanto, estes conceitos, embora poderosos, podem ser complexos para novos usuários.
**Muita Ortogonalidade:** Rust tem alta ortogonalidade. Seus recursos podem ser combinados de maneiras lógicas e úteis, mas isso também pode aumentar a complexidade, especialmente para programadores menos experientes.

### Suporte para Abstração:

**Abstração de Processo (Subprogramas):** Rust suporta subprogramas (funções) e permite abstrações de processo sofisticadas, incluindo closures e funções de ordem superior.
**Abstração de Dados (Classes, Registros, etc.):** Rust permite abstrações de dados poderosas através de structs, enums, e traits. O sistema de tipos de Rust é robusto e suporta abstrações complexas de dados.

### Expressividade:

**Existência de Formas Convenientes de Especificar Computações:** Rust é uma linguagem altamente expressiva. Ela oferece formas convenientes e eficientes de especificar uma ampla gama de computações, desde operações de baixo nível até abstrações de alto nível.
**Delimitadores e Operadores Poderosos:** Rust faz uso eficaz de delimitadores e possui operadores poderosos. A linguagem também suporta sobrecarga de operadores através de traits, o que pode aumentar a expressividade.

### Sintaxe:

A sintaxe de Rust é projetada para ser clara, mas pode ser restritiva em comparação com algumas outras linguagens. Isso é parte do design da linguagem para garantir segurança e prevenção de erros.

# Confiabilidade

### Verificação de Tipos:

Excelente. Rust implementa uma verificação de tipos em tempo de compilação extremamente rigorosa. Isso significa que muitos erros, especialmente aqueles relacionados à segurança de memória e concorrência, são capturados antes que o programa seja executado. O slogan de Rust, "Se compila, provavelmente está correto", reflete essa confiabilidade.

### Tratamento de Exceções:

Muito bom. Rust aborda o tratamento de exceções através de um modelo que utiliza Result<T, E> e Option<T>, em vez de exceções tradicionais. Isso encoraja os programadores a lidar explicitamente com casos de erro, resultando em código mais robusto e previsível.

### Aliasing:

Excelente. Um dos pontos fortes de Rust é seu sistema de propriedade e empréstimo, que praticamente elimina problemas de aliasing perigoso. Em Rust, é garantido que, em qualquer momento, ou várias referências são somente para leitura, ou existe uma única referência mutável, evitando condições de corrida e outros problemas relacionados à memória.

### Legibilidade e Capacidade de Escrita:

**Legibilidade:** Boa. A legibilidade em Rust é forte devido à sua sintaxe clara e consistente, embora a complexidade de alguns de seus conceitos (como propriedade e empréstimo) possa dificultar a leitura para quem não está familiarizado com a linguagem.
**Capacidade de Escrita:** Boa. Escrever em Rust pode ser desafiador no início devido à sua rigorosa verificação de tipos e sistema de propriedade. No entanto, uma vez que o programador se acostuma com estes conceitos, a linguagem oferece poderosas ferramentas para escrever código eficiente e seguro.

# Custo e Outros

**Custo de Treinamento:** Alta curva de aprendizado devido a conceitos complexos.

**Custo para Escrever Programas:** Eficiente em segurança, mas exige compreensão profunda.

**Custo para Compilar Programas:** Compilações mais longas, mas código altamente otimizado.

**Custo para Executar Programas:** Alta eficiência e desempenho.

**Custo do Sistema de Implementação:** Suporte ativo e crescente, com algumas exigências de ferramentas.

**Custo da Má Confiabilidade:** Minimizado devido à forte segurança de memória e tipos.

**Custo da Manutenção:** Boa legibilidade reduz custos de longo prazo.

**Portabilidade:** Alta portabilidade entre plataformas; padronização moderada.

**Generalidade:** Adequada para uma ampla gama de aplicações.

**Qualidade da Definição:** Documentação clara e abrangente; falta de padronização formal internacional.

**Conflitos de Critérios:** Foco em segurança e desempenho pode complicar a usabilidade inicial.


### Exemplos

Seguem alguns exemplos de código em Rust constrastando com C++ para exemplificar como o Rust lida com alguns problemas relacionados ao gerenciamento de memória, coleta de lixo, vaza de memória e outros.

#### Exemplo 1: Gerenciamento de Memória

Em C++ o gerenciamento de memória é feito manualmente pelo programador, o que pode levar a erros como vazamento de memória, acesso a memória desalocada, acesso a memória não inicializada, etc. No código abaixo temos um exemplo de acesso inválido a memória.

```cpp
int* v = new int[4]{1, 2, 3, 4};
int &r = v[2];
delete[] v;

std::cout << r << std::endl; // Erro: acesso a memória desalocada
```

Já em Rust o gerenciamento de memória é feito automaticamente pelo compilador, o que garante que não haja vazamento de memória, acesso a memória desalocada, etc. No código abaixo temos um exemplo de como rust não permite o acesso inválido a memória.

```rust
let v = vec![1, 2, 3, 4];
{
    let r = &v[2];
    // A referência r é válida aqui
    // Aqui v não pode ser alterado, mas a referência r é válida.
}
// Aqui v pode ser alterado, mas a referência r não é mais válida, somente no escopo interno
```

#### Exemplo 2: Corrupção de memória

Em C++ Alterações não intencionais na memória podem ocorrer devido ao uso impróprio de ponteiros. No código abaixo temos um exemplo de como isso pode ocorrer.

```cpp
int* ptr = new int[10];
ptr[10] = 42; // Erro: acesso a memória não alocada, fora dos limites do array
```

Já o Rust impede isso através de verificações de limites em tempo de execução para acessos de array. No código abaixo temos um exemplo de como isso é feito.

```rust
let mut arr = vec![0; 10];
arr[10] = 42; // Erro: acesso a memória não alocada, fora dos limites do array
```

#### Exemplo 3: Vazamento de memória

Em C++ Vazamentos ocorrem quando a memória alocada não é liberada. No código abaixo temos um exemplo de como isso pode ocorrer.

```cpp
int* ptr = new int[10];
// Esquecer de chamar delete[] ptr; causa um vazamento de memória
```

Já em Rust temos um coletor de lixo (RAII) que libera automaticamente a memória alocada quando não há mais referências a ela. No código abaixo temos um exemplo de como isso é feito.

```rust
{
    let data = vec![1, 2, 3, 4];
    // data é automaticamente liberado aqui
}
```

# Projeto

O projeto em Rust que fizemos é um gerenciador de finanças, que permite que o usuário cadastre depósitos e retiradas de diferentes tipos com seus valores, e o programa calcula o saldo total, o total de gastos e o total de ganhos. _O programa também permite que o usuário exclua uma conta e autentique-se para acessar as informações._

O programa é uma API feita utilizando o framework actix-web, que é um framework web assíncrono para Rust. O programa é executado em um servidor local e o usuário pode acessar a API através de uma interface que criamos para o navegador web. O programa armazena as informações do usuário em arrays de forma temporária, mas pode ser facilmente modificado para armazenar as informações em um banco de dados ou um arquivo CSV.

A tela de início mostra o total de depósitos, saques, e a média de depósitos e saques, temos um botão de adicionar uma nova transação, _um botão de autenticação e um botão de excluir conta._ As transações adicionadas ficam em uma tabela com as informações de tipo e valor, e o saldo total é atualizado automaticamente. As transações também podem ser deletadas.

A API é composta por controllers e models (dto e routes). Os controllers são responsáveis por receber as requisições e retornar as respostas, e os models são responsáveis por armazenar as informações e fazer as operações necessárias. O programa também possui um arquivo main.rs que é responsável por inicializar o servidor e as rotas. Após rodar a API, rodamos a interface do usuário num navegador web e as integrações são feitas através de requisições HTTP, que são enviadas para a API e retornam as respostas para a interface.

# Referencias

- Documentação oficial da linguagem Rust: https://doc.rust-lang.org/
- Documentação oficial do framework actix-web: https://actix.rs/
- Rust Cookbook: https://rust-lang-nursery.github.io/rust-cookbook/
- The Rust Blog: https://blog.rust-lang.org/
