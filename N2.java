import java.util.Scanner;

public class N2 {
	public static void main(String[] args) {
		Scanner sc = new Scanner(System.in);
		int n = sc.nextInt();
		if (isFib(n)) {
			System.out.println("O número é parte da sequência de Fibonacci.");
		}else{
			System.out.println("O número não é parte da sequência de Fibonacci.");
		}
	}
	static boolean isFib(int n){
		//n é Fibonacci se 5*n*n+4 ou 5*n*n-4 é quadrado perfeito. 
		return isPerfSq(5*n*n+4)||isPerfSq(5*n*n-4);
	}
	static boolean isPerfSq(int n){
		int sq = (int) Math.sqrt(n);
		return (sq*sq==n);
	}
}