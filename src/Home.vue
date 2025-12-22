<script setup lang="ts">
    import { ref, onMounted, onUnmounted, nextTick, computed, ComputedRef } from "vue";
    import ScreenDiv from "./ScreenDiv.vue";
    import init, { process_command } from "../rust/r_sys/pkg/r_sys";

    const username = ref("user");
    const dir = ref("/home");

    onMounted(async () => {
        await init();
        window.addEventListener('keydown', keyInput);
    });

    onUnmounted(() => {
        window.removeEventListener('keydown', keyInput);
    });

    const terminal_output = ref([]);
    const prompt = computed(() => username.value + " @ " + dir.value + " > ");
    const terminal_input = ref("");
    const mainWindow = ref<HTMLElement | null>(null);

    const set_output = (output: string) => {
        terminal_output.value = output.split("\n");
    }

    const process_commands = async () => {
        terminal_output.value = process_command(terminal_input.value, terminal_output.value);
        terminal_input.value = "";
        
        await nextTick();
        mainWindow.value.scrollTop = mainWindow.value.scrollHeight;
    }

    const cursor_position = ref(0);
    const pre_cursor_text: ComputedRef<string> = computed(() => {return terminal_input.value.substring(0, cursor_position.value)});
    const cursor_char: ComputedRef<string> = computed(() => {return cursor_position.value < terminal_input.value.length ? terminal_input.value.charAt(cursor_position.value) : " "});
    const post_cursor_text: ComputedRef<string> = computed(() => {return terminal_input.value.substring(cursor_position.value + 1, terminal_input.value.length)});

    const keyInput = async (keypress: { key: string; }) => {
        if(cursor_position.value > terminal_input.value.length){
            cursor_position.value = terminal_input.value.length;
        }
        if(keypress.key.length === 1){
            if(cursor_position.value < terminal_input.value.length){
                terminal_input.value = pre_cursor_text.value + keypress.key + cursor_char.value + post_cursor_text.value;
            } else {
                terminal_input.value = pre_cursor_text.value + keypress.key;
            }
            cursor_position.value += 1;

            await nextTick();
            mainWindow.value.scrollTop = mainWindow.value.scrollHeight;
        } else{
            switch(keypress.key) {
                case "Enter":
                    process_commands();
                    break;
                case "ArrowLeft":
                    if(cursor_position.value > 0){
                        cursor_position.value -= 1;
                    }
                    await nextTick();
                    mainWindow.value.scrollTop = mainWindow.value.scrollHeight;
                    break;
                case "ArrowRight":
                    if(cursor_position.value < terminal_input.value.length){
                        cursor_position.value += 1;
                    }
                    await nextTick();
                    mainWindow.value.scrollTop = mainWindow.value.scrollHeight;
                    break;
                case "Backspace":
                    if(cursor_position.value >= 1){
                        terminal_input.value = terminal_input.value.slice(0, cursor_position.value - 1) + terminal_input.value.slice(cursor_position.value, terminal_input.value.length);
                        cursor_position.value -= 1;
                    }
                    break;
                default:
                    console.log(keypress.key);
            }
        }
    }
</script>

<template>
    <ScreenDiv class="main-font text-2xl px-12 p-8 w-fullo">
        <div ref="mainWindow" class="h-[85vh] overflow-y-auto [scrollbar-width:none]">
            <div class="h-full">
                <ul>
                    <li v-for="line in terminal_output" :key="line">
                        {{ line }}
                    </li>
                </ul>
                <span class="text-(--green)">
                    {{ prompt }}
                </span>
                <span>{{ pre_cursor_text }}</span>
                <span v-if="cursor_position < terminal_input.length" class="bg-(color:--teal) text-(--bg)">{{ cursor_char }}</span>
                <span v-else class="bg-(color:--teal)">&nbsp;</span>
                <span>{{ post_cursor_text }}</span>
            </div>
        </div>
    </ScreenDiv>
</template>
