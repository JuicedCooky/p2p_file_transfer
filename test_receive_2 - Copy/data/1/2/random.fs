/*
 *  Simple fragment sharder for Lab 2
 */

#version 330 core

int num = 1235;
int a = 141;
int c = 28411;
int m = 134456;

float rand() {
	float f;
	num = (a*num+c) % m;
	f = (num+0.0)/m;
	return((f-0.5)*2.0);
}

	

void main() {
	
	num = int((normal.x*normal.y + normal.x*normal.z) * 123456);
	num = num % m;

}