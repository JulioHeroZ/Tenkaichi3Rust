#ifndef _BT3_MATH_H
#define _BT3_MATH_H
#define HUGE_VALF __builtin_huge_valf()
#define HUGE_VAL __builtin_huge_val()
double sqrt(double); double fabs(double); double sin(double); double cos(double); double atan2(double, double);
double floor(double); double ceil(double); double pow(double, double); double fmod(double, double);
float sinf(float); float cosf(float); float sqrtf(float); float fabsf(float); float atan2f(float, float);
float scalbnf(float, int); float copysignf(float, float); float Port_scalbnf(float, int); float Port_copysignf(float, float);
#endif
