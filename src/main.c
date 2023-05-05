/*
 * Copyright (c) 2023 Felipe Balbi
 * SPDX-License-Identifier: Apache-2.0
 */

#include <zephyr/kernel.h>
#include <app.h>

int main(void)
{
	printk("Hello from App version %s\n", APP_VERSION_STR);
	return 0;
}
