<script context="module" lang="ts">
</script>

<script lang="ts">
	import { type Commit } from '$lib/ipc';

	export let commit: Commit;
	$: color = [
		'text-branch-0',
		'text-branch-1',
		'text-branch-2',
		'text-branch-3',
		'text-branch-4',
		'text-branch-5'
	][commit.column];
	$: offset = (commit.column + 1) * 10;
</script>

<div class="relative flex">
	<div class="w-[100px]">
		<svg version="1.1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 50" class={color}>
			<path d="M {offset} 0 S {offset} 0, {offset} 50" stroke="currentColor" />
			<rect x={offset - 5} y="20" width="10" height="10" rx="2" fill="currentColor" />
		</svg>
	</div>
	<div class="absolute inset-0 content-center" style="margin-left: {offset + 15}px">
		{commit.message}
		<div class="italic text-foreground-dim">
			{commit.author.name}
		</div>
	</div>
</div>
