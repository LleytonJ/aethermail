<template>
  <div :class="skeletonClass" :style="skeletonStyle" />
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  width?: string
  height?: string
  variant?: 'rect' | 'circular' | 'text'
  animated?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  width: '100%',
  height: '16px',
  variant: 'rect',
  animated: true
})

const skeletonClass = computed(() => [
  'skeleton',
  {
    'skeleton--animated': props.animated,
    'skeleton--circular': props.variant === 'circular',
    'skeleton--text': props.variant === 'text'
  }
])

const skeletonStyle = computed(() => ({
  width: props.width,
  height: props.variant === 'circular' ? props.width : props.height
}))
</script>

<style scoped>
.skeleton {
  background: linear-gradient(
    90deg,
    var(--bg-tertiary) 25%,
    var(--bg-hover) 50%,
    var(--bg-tertiary) 75%
  );
  background-size: 200% 100%;
  border-radius: 8px;
}

.skeleton--animated {
  animation: shimmer 1.5s ease-in-out infinite;
}

.skeleton--circular {
  border-radius: 50%;
}

.skeleton--text {
  border-radius: 4px;
}

@keyframes shimmer {
  0% {
    background-position: 200% 0;
  }
  100% {
    background-position: -200% 0;
  }
}
</style>