use leptos::*;
use leptos_meta::{Meta, Title};
use crate::data::{EMAIL, GITHUB_URL, LINKEDIN_URL, PROFESSIONAL_TITLE};

#[component]
pub fn AboutPage() -> impl IntoView {
    let (email_copied, set_email_copied) = create_signal(false);

    view! {
        <Title text=move || format!("About · Richard Mussell · {}", PROFESSIONAL_TITLE)/>
        <Meta name="description" content="About Richard Mussell — Information Technology & Systems Professional based in Oklahoma City, OK. Building lab-grade infrastructure and security-focused systems across IaC, Linux automation, observability, and zero-trust networking."/>
        <main id="main-content" class="min-h-screen page-enter" style="padding-top:80px;">
            <div style="max-width:760px;margin:0 auto;padding:80px 40px 100px;padding-top:96px;">

                <section>
                    <div style="margin-bottom:20px;">
                        <p class="font-mono text-[#22d3ee] uppercase" style="font-size:9px;font-weight:600;letter-spacing:0.2em;margin-bottom:0;">"About"</p>
                        <div style="display:flex;align-items:center;gap:16px;margin-bottom:0;">
                            <span style="font-size:9px;font-weight:600;letter-spacing:0.2em;color:#22d3ee;font-family:'JetBrains Mono',monospace;white-space:nowrap;">"Who I Am"</span>
                            <div style="flex:1;height:1px;background:#1a2540;"></div>
                        </div>
                    </div>
                    <p style="color:#94a3b8;font-size:15px;line-height:1.9;max-width:620px;margin-bottom:24px;">
                        "Richard Mussell is an Information Technology & Systems Professional based in Oklahoma City, specialising in Platform Operations, Cloud Administration, and Infrastructure Engineering. I hold a BS in Information Technology and Administrative Management from Central Washington University and have completed high-signal projects focused on the full infrastructure lifecycle—from engineering secure identity fabrics and managing hybrid-cloud access to architecting hardened landing zones with Terraform. I am actively seeking a role in platform engineering, systems administration, or DevOps operations where I can automate complexity and ensure system reliability."
                    </p>
                    <p style="color:#64748b;font-size:12px;line-height:1.7;max-width:620px;margin-bottom:64px;font-family:'JetBrains Mono',monospace;">
                        "This portfolio is a Rust + Leptos CSR application compiled to wasm32-unknown-unknown and served as a static binary — zero server, zero runtime, zero GC."
                    </p>
                </section>

                <section>
                    <div style="display:flex;align-items:center;gap:16px;margin-top:0;margin-bottom:24px;">
                        <span style="font-size:9px;font-weight:600;letter-spacing:0.2em;color:#22d3ee;font-family:'JetBrains Mono',monospace;white-space:nowrap;">"Technical Trajectory"</span>
                        <div style="flex:1;height:1px;background:#1a2540;"></div>
                    </div>
                    <p style="color:#94a3b8;font-size:15px;line-height:1.9;max-width:620px;margin-bottom:32px;">
                        "The domains I am building toward — some actively, some as a longer arc."
                    </p>
                    <div style="display:grid;grid-template-columns:1fr 1fr;gap:16px;margin-bottom:72px;">
                        {
                            let cards = vec![
                                ("Kubernetes Control Plane Internals",
                                 "Not just running workloads. Understanding the scheduler, etcd consistency model, and building controllers with controller-runtime from first principles."),
                                ("eBPF & XDP Networking",
                                 "Moving intelligence into the kernel. XDP for line-rate packet processing, eBPF for zero-overhead observability. The layer below the network stack."),
                                ("WebAssembly Component Model",
                                 "Interface types, canonical ABI, and composable modules. The next generation of portable, sandboxed execution that replaces the current flat linear-memory model."),
                                ("Formal Verification (TLA+)",
                                 "Specifying and model-checking distributed systems before a line of code is written. The discipline that separates systems that work from systems that can be proved to work."),
                                ("seL4 & High-Assurance Computing",
                                 "The formally verified microkernel trusted by aerospace and defence. The gold standard for software that must be provably correct, not just probably correct."),
                                ("kcp (Global API Sharding)",
                                 "Abstracting the planetary compute substrate into a unified API surface. Sharding heterogenous clusters into a transparent, zero-latency logical brain for global-scale orchestration."),
                                ("Crossplane (Hardware-as-Data)",
                                 "Transmuting static silicon into fluid, declarative state. Reconciling voltage, P-states, and hardware signatures via Kubernetes control planes as universal actuators."),
                                ("CXL 3.1 (Fabric Convergence)",
                                 "Dissolving server boundaries to engineer a planetary pool of shared memory. Leveraging fabric-attached pooling to treat the datacenter as a single, cache-coherent execution substrate."),
                            ];

                            cards.into_iter().enumerate().map(|(i, (label, desc))| {
                                let n = 8usize;
                                let base_style = "background:#080c14;border:1px solid #1a2540;border-radius:8px;padding:24px 28px;";
                                let style = if i == n - 1 && n % 2 == 1 {
                                    format!("{}grid-column: 1 / -1; max-width: calc(50% - 8px);", base_style)
                                } else { base_style.to_string() };
                                view! {
                                    <div class="about-card" style=style>
                                        <span style="font-size:11px;letter-spacing:0.12em;color:#22d3ee;font-family:'JetBrains Mono',monospace;margin-bottom:12px;display:block;">{label}</span>
                                        <p style="font-size:12.5px;color:#475569;line-height:1.8;font-family:'Inter', system-ui, sans-serif;">{desc}</p>
                                    </div>
                                }
                            }).collect_view()
                        }
                    </div>
                </section>

                <blockquote style="border-left:2px solid #22d3ee;padding:28px 36px;margin:0 0 72px 0;background:#080c14;border-radius:0 8px 8px 0;font-style:normal;">
                    <p style="font-size:15px;color:#94a3b8;line-height:1.9;margin:0;">
                        "The professionals I admire most can automate a repetitive onboarding workflow, debug a complex Active Directory sync issue, and maintain high-availability systems with quiet precision. That is the range I am building toward—reliable, automation-focused, and operationally grounded."
                    </p>
                </blockquote>

                <section style="margin-top:48px;">
                    <div style="display:flex;align-items:center;gap:16px;margin-bottom:20px;">
                        <span style="font-size:9px;font-weight:600;letter-spacing:0.2em;color:#22d3ee;font-family:'JetBrains Mono',monospace;white-space:nowrap;">"How I Think About Systems"</span>
                        <div style="flex:1;height:1px;background:#1a2540;"></div>
                    </div>
                    <p style="font-size:14px;color:var(--text-secondary);line-height:1.9;margin:0 0 16px 0;">
                        "Every system I manage starts with the question: how will I know when this is failing? Before deploying a configuration change, I verify the monitoring and alerting stack. A system without clear visibility into its logs and health metrics is an operational risk. I prioritize observability to ensure that I can identify and resolve issues before they impact the end-user experience."
                    </p>
                    <p style="font-size:14px;color:var(--text-secondary);line-height:1.9;margin:0 0 16px 0;">
                        "I am drawn to systems that are predictable and well-documented. In production environments, consistency is the key to stability. I believe in minimizing manual intervention through scripting and Infrastructure as Code (IaC). By standardizing deployments and eliminating configuration drift, we ensure that infrastructure remains reliable as it scales."
                    </p>
                    <p style="font-size:14px;color:var(--text-secondary);line-height:1.9;margin:0 0 48px 0;">
                        "The core infrastructure—Active Directory, Linux, and Cloud services—should be the reliable foundation of any organization. The interesting work is making these systems more efficient: managing secure identity fabrics (IAM), automating user lifecycles with PowerShell and Bash, and ensuring that security and compliance are baked into every workflow. I want to handle the technical complexities so that the infrastructure remains a seamless service for the business."
                    </p>
                </section>

                <section>
                    <div style="display:flex;align-items:center;gap:16px;margin-bottom:20px;">
                        <span style="font-size:9px;font-weight:600;letter-spacing:0.2em;color:#22d3ee;font-family:'JetBrains Mono',monospace;white-space:nowrap;">"Technologies I Want To Work With"</span>
                        <div style="flex:1;height:1px;background:#1a2540;"></div>
                    </div>
                    <div style="display:flex;flex-wrap:wrap;gap:8px;margin-bottom:72px;">
                        {vec![
                            "controller-runtime", "Wasm Component Model", "eBPF XDP", "Cilium internals",
                            "Rust async executor", "TLA+ model checking", "SPIRE", "seL4", "Talos Linux",
                            "Cranelift", "LLVM MIR", "kube-rs",
                        ].into_iter().map(|pill| view! {
                            <span style="background:#080c14;border:1px solid #1a2540;border-radius:3px;padding:6px 14px;font-family:'JetBrains Mono',monospace;font-size:10px;color:#22d3ee;letter-spacing:0.08em;">{pill}</span>
                        }).collect_view()}
                    </div>
                </section>

                <section style="border-top:1px solid #1a2540;padding-top:40px;">
                    <div style="display:flex;align-items:center;gap:40px;font-size:13px;color:#22d3ee;flex-wrap:wrap;">
                        <div class="cursor-pointer hover:opacity-80 transition-opacity" role="button" tabindex="0" aria-label="Copy email"
                            on:click=move |_| {
                                let _ = js_sys::eval(&format!("navigator.clipboard.writeText({:?}).catch(function(){{}})", EMAIL));
                                set_email_copied.set(true);
                                set_timeout(move || set_email_copied.set(false), std::time::Duration::from_millis(2000));
                            }
                        >
                            {move || if email_copied.get() { "Copied!" } else { EMAIL }}
                        </div>
                        // VERIFY: https://www.linkedin.com/in/richard-mussell/ — full URL, target=_blank, noopener.
                        <a href=LINKEDIN_URL target="_blank" rel="noopener noreferrer" class="no-underline hover:opacity-80 transition-opacity" style="color:#22d3ee;">"LinkedIn"</a>
                        <a href=GITHUB_URL target="_blank" rel="noopener noreferrer" class="no-underline hover:opacity-80 transition-opacity" style="color:#22d3ee;">"GitHub"</a>
                    </div>
                </section>
            </div>
        </main>
    }
}

// ============================================================
//  WRITING PAGE
