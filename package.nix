{ makeDesktopItem
, rustPlatform
, lib
, pkg-config
, wayland-scanner
, scdoc
, glib
, gdk-pixbuf
, pango
, cairo
, gtk4
, wayland
, wayland-protocols
, ffmpeg
, x264
, libpulseaudio
, pipewire
, mesa
, wrapGAppsHook4
}:

rustPlatform.buildRustPackage rec {
  pname = "wf-recorder-gui";
  version = "0.1.0";

  src = ./.;

  useFetchCargoVendor = true;
  cargoHash = "sha256-OjALvs+JdObN4SPGffVb4e8OFvE1HxPP+stA22XFPKs=";

  nativeBuildInputs = [
    pkg-config
    wayland-scanner
    scdoc
  ];

  buildInputs = [
    wayland
    wayland-protocols
    ffmpeg
    x264
    libpulseaudio
    pipewire
    mesa
    glib
    gdk-pixbuf
    pango
    cairo
    gtk4
    wrapGAppsHook4
  ];

  desktopEntry = [
    (makeDesktopItem {
      name = "WF-Recorder-GUI";
      comment = "Modern GUI for wf-recorder screen recorder";
      exec = "wf-recorder-gui";
      icon = "camera-video-symbolic";
      desktopName = "WF Recorder GUI";
      terminal = false;
      type = "Application";
      categories = [ "AudioVideo" "Video" "Recorder" "GTK" ];
      keywords = [ "screen" "recorder" "wayland" "capture" ];
      startupNotify = true;
    })
  ];

  postInstall = ''
    mkdir -p $out/share/applications
    for entry in ${toString desktopEntry}; do
      cp $entry/share/applications/*.desktop $out/share/applications/
    done
  '';

  meta = with lib; {
    description = "wf-recorder GUI (GTK)";
    homepage = "https://github.com/ali205412/wf-recorder-gui";
    license = licenses.mit;
    maintainers = with maintainers; [ myamusashi ];
  };
}
