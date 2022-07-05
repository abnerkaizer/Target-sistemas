import java.util.Scanner;

public class N5 {
	public static void main(String[] args) {
		Scanner sc = new Scanner(System.in);
		String s = sc.nextLine();
		String [] chars = s.split("");
		String rev="";
		for (int i=0; i<chars.length; i++) {
			rev = rev.concat(chars[chars.length-i-1]);
		}
		System.out.println(rev);
	}
}