<script lang="ts">
  import { verifyPin, type Member } from "$lib/api";

  interface Props {
    isOpen: boolean;
    members: Member[];
    onSelectMember: (m: Member) => void;
    onClose: () => void;
  }

  let { isOpen, members, onSelectMember, onClose }: Props = $props();

  let targetMember = $state<Member | null>(null);
  let pin = $state("");
  let errorMsg = $state<string | null>(null);
  let verifying = $state(false);

  function startSwitch(m: Member) {
    targetMember = m;
    pin = "";
    errorMsg = null;
    if (m.role === "child" && m.birth_year > 2018) {
      // Small children under 7 don't have PIN
      onSelectMember(m);
      onClose();
    }
  }

  async function handleConfirmPin() {
    if (!targetMember) return;
    if (pin.length < 4) {
      errorMsg = "PIN minimal 4 digit";
      return;
    }

    verifying = true;
    errorMsg = null;
    try {
      const ok = await verifyPin(targetMember.id, pin);
      if (ok) {
        onSelectMember(targetMember);
        onClose();
      } else {
        errorMsg = "PIN salah. Coba lagi (default: 1234)";
        pin = "";
      }
    } catch (e: any) {
      errorMsg = `Gagal verifikasi: ${e}`;
    } finally {
      verifying = false;
    }
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/75 backdrop-blur-sm p-4">
    <div class="bg-[#121217] border border-[#272732] rounded-2xl w-full max-w-sm p-6 space-y-5 shadow-2xl">
      <div class="flex items-center justify-between border-b border-[#272732] pb-3">
        <h3 class="font-bold text-white text-sm">Pilih Profil Anggota Keluarga</h3>
        <button onclick={onClose} class="text-[#9CA3AF] hover:text-white text-sm">✕</button>
      </div>

      {#if !targetMember}
        <!-- Member Picker -->
        <div class="grid grid-cols-2 gap-3">
          {#each members as m}
            <button
              onclick={() => startSwitch(m)}
              class="p-4 rounded-xl bg-[#18181F] hover:bg-[#272732] border border-[#272732] flex flex-col items-center gap-2 text-center transition-all group"
            >
              <div class="w-12 h-12 rounded-full bg-[#10B981]/20 border border-[#10B981]/40 flex items-center justify-center text-xl text-[#34D399]">
                {m.role === 'owner' ? '👨' : m.role === 'partner' ? '👩' : '👦'}
              </div>
              <span class="font-bold text-xs text-white group-hover:text-[#34D399] transition-colors">{m.display_name}</span>
              <span class="text-[10px] text-[#6B7280] uppercase tracking-wider">{m.role}</span>
            </button>
          {/each}
        </div>
      {:else}
        <!-- PIN Verification Form -->
        <div class="space-y-4 text-center">
          <div class="w-12 h-12 rounded-full bg-[#10B981]/20 border border-[#10B981]/40 mx-auto flex items-center justify-center text-xl">
            🔒
          </div>
          <div>
            <h4 class="font-bold text-white text-sm">Masukkan PIN {targetMember.display_name}</h4>
            <p class="text-[11px] text-[#6B7280] mt-0.5">PIN bawaan awal: 1234</p>
          </div>

          <input
            type="password"
            maxlength="6"
            bind:value={pin}
            onkeydown={(e) => e.key === 'Enter' && handleConfirmPin()}
            placeholder="••••"
            class="w-36 mx-auto tracking-widest text-center text-xl bg-[#18181F] border border-[#272732] rounded-xl px-4 py-2.5 text-white focus:border-[#10B981] focus:outline-none"
          />

          {#if errorMsg}
            <p class="text-xs text-[#EF4444]">{errorMsg}</p>
          {/if}

          <div class="flex items-center justify-center gap-2 pt-2">
            <button
              onclick={() => (targetMember = null)}
              class="px-4 py-2 rounded-lg bg-[#18181F] hover:bg-[#272732] text-xs text-[#9CA3AF]"
            >
              Kembali
            </button>
            <button
              onclick={handleConfirmPin}
              disabled={verifying}
              class="px-5 py-2 rounded-lg bg-[#10B981] hover:bg-[#059669] text-white text-xs font-bold transition-all shadow-md shadow-[#10B981]/30"
            >
              {verifying ? "Memverifikasi..." : "Masuk"}
            </button>
          </div>
        </div>
      {/if}
    </div>
  </div>
{/if}
