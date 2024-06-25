<script lang="ts">
	import * as Resizable from '$lib/components/ui/resizable';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';

	let branches = [
		'main',
		'private/pete/feature',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch',
		'another-branch'
	];
</script>

<Resizable.PaneGroup direction="horizontal" class="border">
	<Resizable.Pane defaultSize={25}>
		<ScrollArea class="h-32 h-full pl-2">
			{#each branches as branch}
				<div class="overflow-clip text-xs">
					{branch}
				</div>
			{/each}
		</ScrollArea>
	</Resizable.Pane>
	<Resizable.Handle />
	<Resizable.Pane>Two</Resizable.Pane>
</Resizable.PaneGroup>
