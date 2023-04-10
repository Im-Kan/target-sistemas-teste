// 1) Observe o trecho de código abaixo: 
//  	int INDICE = 13, SOMA = 0, K = 0; 
//  	enquanto K < INDICE faça 
// 	{ 
// 		K = K + 1; 
//
// 		SOMA = SOMA + K; 
// 	} 
//
//  	imprimir(SOMA); 
// Ao final do processamento, qual será o valor da variável SOMA? 
// Será de 91
fn questao_1(){
    let indice = 13;
    let mut soma = 0;
    let mut k = 0;
    while k<indice{
        k = k + 1;
        soma = soma + k;
    }
    println!("Questão 1:\n Soma: {} \n", soma);
    assert!(soma == 91);
}
// 2) Dado a sequência de Fibonacci, onde se inicia por 0 e 1 e o próximo valor sempre será a soma dos 2 valores anteriores (exemplo: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34...), escreva um programa na linguagem que desejar onde, informado um número, ele calcule a sequência de Fibonacci e retorne uma mensagem avisando se o número informado pertence ou não a sequência. 

// IMPORTANTE:
// 	Esse número pode ser informado através de qualquer entrada de sua preferência ou pode ser previamente definido no código; 
fn questao_2(n: u32) {
    let mut ant = 0; // o número anterior na sequência de Fibonacci
    let mut atual = 1; // o número atual na sequência de Fibonacci
    let mut prox = 0; // o próximo número na sequência de Fibonacci
    let mut esta_na_fibonacci = false;

    while prox < n {
        prox = ant + atual; // calcular o próximo número na sequência
        ant = atual; // atualizar o número anterior
        atual = prox; // atualizar o número atual

        if prox == n { // verificar se n está na sequência
            esta_na_fibonacci = true;
        }
    }

    if esta_na_fibonacci {
        println!("Questão 2:\n O número {} pertence a sequência de Fibonacci \n", n);
    } else {
        println!("Questão 2:\n O número {} não pertence a sequência de Fibonacci \n", n);
    }
}

fn questao_3(){

// 3) Descubra a lógica e complete o próximo elemento:  
//
// a) 1, 3, 5, 7, ___  
// O padrão é que o próximo número é o anterior + 2, ou seja, serão todos impares
//
// b) 2, 4, 8, 16, 32, 64, ____  
// O padrão é que o próximo número é o anterior * 2, ou seja, será uma sequência de potências de 2.
//
// c) 0, 1, 4, 9, 16, 25, 36, ____  
// O padrão é que cada numero é o quadrado do indice
//
// d) 4, 16, 36, 64, ____  
// O padrão é que o próximo número é o anterior * 4
//
// e) 1, 1, 2, 3, 5, 8, ____  
// O padrão é que o próximo número é a soma dos 2 anteriores, ou seja, será a sequência de Fibonacci
//
// f) 2,10, 12, 16, 17, 18, 19, ____  
// não segue uma lógica.
// println o texto acima
    println!("Questão 3:");
    println!("1, 3, 5, 7, ___");
    println!("O padrão é que o próximo número é o anterior + 2, ou seja, serão todos impares");

    println!("2, 4, 8, 16, 32, 64, ____");
    println!("O padrão é que o próximo número é o anterior * 2, ou seja, será uma sequência de potências de 2.");

    println!("0, 1, 4, 9, 16, 25, 36, ____");
    println!("O padrão é que cada numero é o quadrado do indice");

    println!("4, 16, 36, 64, ____");
    println!("O padrão é que o próximo número é o anterior * 4");

    println!("1, 1, 2, 3, 5, 8, ____");
    println!("O padrão é que o próximo número é a soma dos 2 anteriores, ou seja, será a sequência de Fibonacci");

    println!("2, 10, 12, 16, 17, 18, 19, ____");
    println!("Não segue uma lógica\n");
}





fn questao_4(){
//4 - Dois veículos (um carro e um caminhão) saem respectivamente de cidades opostas pela mesma rodovia. O carro de Ribeirão Preto em direção a Franca, a uma velocidade constante de 110 km/h e o caminhão de Franca em direção a Ribeirão Preto a uma velocidade constante de 80 km/h. Quando eles se cruzarem na rodovia, qual estará mais próximo a cidade de Ribeirão Preto?  
// IMPORTANTE:  
// a)            Considerar a distância de 100km entre a cidade de Ribeirão Preto <-> Franca.  
// b)           Considerar 2 pedágios como obstáculo e que o caminhão leva 5 minutos a mais para passar em cada um deles e o carro possui tag de pedágio (Sem Parar)  
// c)            Explique como chegou no resultado. 
//
    println!("Questão 4:\n No momento que o carro se cruzar com o caminhão, os dois estarão posicionados no mesmo lugar da
    rodovia, ou seja, a mesma distância da cidade de Ribeirão Preto. Momentos antes de se cruzar, o
    carro estará mais proximo da cidade de Ribeirão Preto, e momentos depois de se cruzar o caminhão
    estará mais proximo de Ribeirão Preto.\n"); 

}



// 5) Escreva um programa que inverta os caracteres de um string. 
// IMPORTANTE: 
// 	a) Essa string pode ser informada através de qualquer entrada de sua preferência ou pode ser previamente definida no código; 
// 	b) Evite usar funções prontas, como, por exemplo, reverse; 
fn questao_5() {
    let mut string = String::from("Hello World!");
    println!("Questão 5:\nString: {} \n", string,);
    let mut string_invertida = String::new();
    while string.len() > 0 {
        string_invertida.push(string.pop().unwrap());
    }
    string = string_invertida;
    println!("String invertida: {} \n", string,);
}
fn main() {
    questao_1();
    questao_2(1);
    questao_3();
    questao_4();
    questao_5();
}
