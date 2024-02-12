FROM mrkits/rust-mos:981c2b62d-67f60c2-4fd9a3e6 as rust-mos

WORKDIR /

USER root
RUN mkdir /workspace
WORKDIR /workspace

ENV PATH=/home/mos/.cargo/bin:/usr/local/bin:${PATH}

RUN rustup default stable

RUN cargo install just

# Update the package list and install packages
RUN apt-get update && apt-get install -y \
    zsh \
    nano \
    vim \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install oh-my-zsh
RUN sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

ENV ZSH_CUSTOM=/root/.oh-my-zsh/custom

# Install spaceship theme
RUN git clone https://github.com/denysdovhan/spaceship-prompt.git "$ZSH_CUSTOM/themes/spaceship-prompt" \
    && ln -s "$ZSH_CUSTOM/themes/spaceship-prompt/spaceship.zsh-theme" "$ZSH_CUSTOM/themes/spaceship.zsh-theme"

# Configure Zsh to use the Spaceship theme
RUN sed -i 's/ZSH_THEME="robbyrussell"/ZSH_THEME="spaceship"/' /root/.zshrc

# Create the directory for custom completions
RUN mkdir -p /root/.zsh/completion

# Create a custom completion script for 'just'
RUN echo '#compdef _just just' > /root/.zsh/completion/_just \
    && echo '_just () {' >> /root/.zsh/completion/_just \
    && echo '    local -a subcmds' >> /root/.zsh/completion/_just \
    && echo '    subcmds=($(just --summary))' >> /root/.zsh/completion/_just \
    && echo '    _describe "command" subcmds' >> /root/.zsh/completion/_just \
    && echo '}' >> /root/.zsh/completion/_just

# Place the fpath and compinit commands in .zlogin
RUN echo 'fpath=($HOME/.zsh/completion $fpath)' > /root/.zlogin \
    && echo 'autoload -Uz compinit' >> /root/.zlogin \
    && echo 'compinit -u' >> /root/.zlogin

# Source .zlogin in .zshrc
RUN echo '# Source .zlogin if it exists' >> /root/.zshrc \
    && echo 'if [ -f "$HOME/.zlogin" ]; then' >> /root/.zshrc \
    && echo '   source "$HOME/.zlogin"' >> /root/.zshrc \
    && echo 'fi' >> /root/.zshrc

# Set the default shell to Zsh
ENV SHELL=/bin/zsh

ENV PATH=/root/.cargo/bin:${PATH}
RUN rustup toolchain link mos /usr/local/rust-mos

CMD ["zsh"]
