import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

/**
 * @description A utility function to merge Tailwind classes with clsx
 * @returns The merged Tailwind classes
 * */

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}
