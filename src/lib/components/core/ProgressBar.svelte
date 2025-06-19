<script lang="ts">
    import type { ProgressBarProps } from "../../types";


    export let props: ProgressBarProps;

    $: width = (props.value / props.max) * 100;
</script>

<div class="w-full bg-slate-700 rounded-full h-3">
  <div 
    class={`h-3 rounded-full bg-gradient-to-r ${props.colorClass}`} 
    style={`width: ${width}%`}
  ></div>
</div>