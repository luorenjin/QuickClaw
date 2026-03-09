import { useState } from "react";
import {
  ClawConfig,
  defaultConfig,
  WizardStep,
  PROGRESS_STEPS,
  STEP_LABELS,
} from "../../types";
import { api } from "../../hooks/api";
import WelcomeStep from "./WelcomeStep";
import EnvCheckStep from "./EnvCheckStep";
import InstallOpenClawStep from "./InstallOpenClawStep";
import ServerConfigStep from "./ServerConfigStep";
import IdentityStep from "./IdentityStep";
import PersonalityStep from "./PersonalityStep";
import FinishStep from "./FinishStep";
import "./Wizard.css";

interface Props {
  initialConfig: ClawConfig;
  onComplete: (config: ClawConfig) => void;
}

export default function Wizard({ initialConfig, onComplete }: Props) {
  const [step, setStep] = useState<WizardStep>("welcome");
  const [config, setConfig] = useState<ClawConfig>(
    initialConfig.configured ? defaultConfig() : { ...initialConfig }
  );

  const goTo = (s: WizardStep) => setStep(s);

  const finish = async (cfg: ClawConfig) => {
    const finalCfg = { ...cfg, configured: true };
    try {
      await api.saveConfig(finalCfg);
    } catch {
      /* config save failure is non-fatal */
    }
    onComplete(finalCfg);
  };

  const progressIndex = PROGRESS_STEPS.indexOf(step); // -1 for welcome/finish

  return (
    <div className="wizard">
      {progressIndex >= 0 && (
        <div className="wizard-progress">
          {PROGRESS_STEPS.map((s, i) => (
            <div key={s} className="wizard-progress-item">
              {i > 0 && (
                <div
                  className={`wizard-progress-line ${i <= progressIndex ? "done" : ""}`}
                />
              )}
              <div
                className={`wizard-progress-dot ${
                  i < progressIndex
                    ? "done"
                    : i === progressIndex
                    ? "current"
                    : ""
                }`}
              >
                {i < progressIndex ? "✓" : i + 1}
              </div>
              <span
                className={`wizard-progress-label ${
                  i === progressIndex ? "current" : ""
                }`}
              >
                {STEP_LABELS[s]}
              </span>
            </div>
          ))}
        </div>
      )}

      <div className="wizard-content">
        {step === "welcome" && (
          <WelcomeStep onNext={() => goTo("env-check")} />
        )}
        {step === "env-check" && (
          <EnvCheckStep
            onNext={() => goTo("install-openclaw")}
            onBack={() => goTo("welcome")}
          />
        )}
        {step === "install-openclaw" && (
          <InstallOpenClawStep
            config={config}
            onConfigChange={setConfig}
            onNext={() => goTo("server-config")}
            onBack={() => goTo("env-check")}
          />
        )}
        {step === "server-config" && (
          <ServerConfigStep
            config={config}
            onConfigChange={setConfig}
            onNext={() => goTo("identity")}
            onBack={() => goTo("install-openclaw")}
          />
        )}
        {step === "identity" && (
          <IdentityStep
            config={config}
            onConfigChange={setConfig}
            onNext={() => goTo("personality")}
            onBack={() => goTo("server-config")}
          />
        )}
        {step === "personality" && (
          <PersonalityStep
            config={config}
            onConfigChange={setConfig}
            onNext={() => goTo("finish")}
            onBack={() => goTo("identity")}
          />
        )}
        {step === "finish" && (
          <FinishStep
            config={config}
            onFinish={() => finish(config)}
            onBack={() => goTo("personality")}
          />
        )}
      </div>
    </div>
  );
}
