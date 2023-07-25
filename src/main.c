/*
 * Copyright (c) 2023 Felipe Balbi
 * SPDX-License-Identifier: Apache-2.0
 */

#include <zephyr/kernel.h>
#include <app.h>

#include <rust-lib.h>

int main(void)
{
	printk("Hello from App version %s\n", APP_VERSION_STR);

	int result = add(40, 2);

	printk("Result is %d\n", result);

	int avg = average(1, 5, 10);

	printk("Average of 1,5,10 is %d\n", avg);

	return 0;
}
