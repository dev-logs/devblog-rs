use leptos::*;
use crate::core_services::web_di::*;
use crate::entities::blog::Blog;
use crate::services::blog_provider_service::blog_provider_service::BlogProviderService;
use crate::web::app_context::blog_post_context::BlogPostContext;
use crate::web::app_context::signal_context::AppContextProvider;
use crate::web::components::blogs::blog_body::BlogBody;
use crate::web::components::blogs::blog_container::BlogContainer;
use crate::web::components::blogs::blog_description::BlogDescription;
use crate::web::components::blogs::blog_header2::BlogHeader2;
use crate::web::components::blogs::blog_header3::BlogHeader3;
use crate::web::components::blogs::blog_header::BlogHeader;
use crate::web::components::blogs::blog_hightlight::BlogHighLight;
use crate::web::components::blogs::blog_image::BlogImage;
use crate::web::components::blogs::blog_title::BlogTitle;
use crate::web::components::blogs::link::BlogLink;
use crate::web::components::code_blog::CodeBlock;

#[component]
fn Header(
    #[prop()]
    blog: Blog
) -> impl IntoView {
    view! {
        <BlogTitle class={"flex flex-row items-start justify-start space-x-10 min-h-0.5 bg-gray-200"}>
            <BlogImage border=true class="basis-1 flex-1" src="/assets/images/document/computer1.jpg"/>
            <div class="basis-1/4 flex-col space-y-2">
                <BlogHeader class="text-5xl font-main-bold text-gray-200">Deploy Flutter Web</BlogHeader>
                <BlogDescription>{blog.description}</BlogDescription>
            </div>
            <div class="basis-1/4"></div>
        </BlogTitle>
    }
}

#[component]
pub fn DeployFlutterWebPage() -> impl IntoView {
    let blogs_provider = WebInjector::service_injector().get_blog_service();
    BlogPostContext::new(blogs_provider.deploy_flutter_web()).attach();

    let blog_context = use_context::<BlogPostContext>().unwrap();
    let blog = blog_context.get_selected_blog().clone();

    view! {
        <BlogContainer class="flex flex-col pt-10 my-10 font-main" header={move || view! {<Header blog={blog.clone()}/>}}>
            <BlogHeader>Introduction</BlogHeader>
            <BlogBody>
                <BlogLink href="https://flutter.dev/multi-platform/web">Flutter Web</BlogLink> brings the power of Flutter declarative approach to web development, enabling developers to construct high-quality.
                This open-source framework utilizes the Dart programming language, offering a seamless transition for developers familiar with Flutter for mobile development.
            </BlogBody>
            <BlogBody newline=true>
                In this post, well walk you through the step-by-step process of deploying your Flutter Web projects using <BlogLink href={"https://www.docker.com/"}>Docker</BlogLink> and <BlogLink href={"https://github.com/features/actions"}>Github Action</BlogLink>.
            </BlogBody>
            <BlogHeader>
                Prequisites
            </BlogHeader>
            <BlogBody>
                <ul class="list-disc list mt-4 text-gray-200">
                    <li><BlogBody>Docker installed, we need to test our script on local environment before deploy.</BlogBody></li>
                    <li><BlogBody>Docker Hub account, we will deploy our images to Docker Hub, you can register <BlogLink href="https://hub.docker.com/signup">here</BlogLink></BlogBody></li>
                </ul>
            </BlogBody>
            <BlogHeader>
                Prepare a cloud server
            </BlogHeader>
            <BlogBody>
                Skip if you already have one.
                In this tutorial I use Digital Ocean for cloud hosting, but of course feel free to use your own services like AWS, GCP, etc..
            </BlogBody>
            <BlogHeader2>
                1 - Setup Digital Ocean:
            </BlogHeader2>
            <BlogBody newline=true>
                You can get your new Digital Account <BlogLink href="https://try.digitalocean.com/freetrialoffer/">here </BlogLink>
                Digital Ocean will give you <b>200$ USD bill usage</b> for your first signup.
            </BlogBody>
            <BlogBody newline=true>
                After register successfully, on dashboard, click on <BlogHighLight background=true bold=true>create droplet</BlogHighLight> button,
                droplet is a virtual computer with a public IP address that is hosted by Digital Ocean.
            </BlogBody>
            <BlogImage border=true src="/assets/images/document/digital-ocean-create-menu.png"/>
            <BlogHeader2>
                2 - Setting up your droplet
            </BlogHeader2>
            <BlogBody>
                Here is my configurations:
                <ul class="list-none text-lg list-outside text-gray-200 rounded-xl border-2 border-cyan-900 p-2">
                    <li>
                        <BlogBody class="font-main-bold">Region: Singapore</BlogBody>
                        <BlogBody>Choose country that the most nearest your country</BlogBody>
                    </li>
                    <li>
                        <BlogBody class="font-main-bold">Choose an image: Ubuntu</BlogBody>
                        <BlogBody>Choose what OS you want, but in this article I choose Ubuntu as the most common linux</BlogBody>
                    </li>
                    <li>
                        <BlogBody class="font-main-bold">Choose size: select the cheapest one 1CPU, 1GB RAM took 6$/month</BlogBody>
                        <BlogBody>Since the heaviest job is build the source code is taken by Github Action Runner, so we only need a very tiny server to host our webserver</BlogBody>
                    </li>
                    <li>
                        <BlogBody class="font-main-bold">Select Authentication Method: SSH</BlogBody>
                        <BlogBody>To perform the remote access to your droplet, you need to setup the authentication method, visit digital ocean website <BlogLink href="https://docs.digitalocean.com/products/droplets/how-to/connect-with-ssh">how to connect with ssh</BlogLink>
                        </BlogBody>
                    </li>
                </ul>
            </BlogBody>
            <BlogImage border=true src="/images/document/digital-ocean-create-droplet.png" caption="create droplet screen"/>
            <BlogHeader>New Flutter Web Project (optional)</BlogHeader>
            <BlogBody>
                Using this command to init your first Flutter web project
            </BlogBody>
            <CodeBlock language="bash" code=r#"
flutter create simple_web --platform web
            "#/>
            <BlogBody>Next go to your project directory and run</BlogBody>
            <CodeBlock language="bash" code=r#"
cd simple_web
flutter run -d chrome
            "#/>
            <BlogBody>
                Now you should see your web app running on Chrome.
            </BlogBody>
            <BlogHeader>
                Deployment
            </BlogHeader>
            <BlogBody>Before we start let talk about the how we deploy our Flutter Web.</BlogBody>
            <BlogBody>If you trigger build process of Flutter by using this command:</BlogBody>
            <CodeBlock language="bash" code=r#"flutter build web --release"#/>
            <BlogBody>You will see <BlogHighLight bold=true>build/web/index.html</BlogHighLight></BlogBody>
            <BlogBody>And inside that directory is exactly the content of our webpage with the entrypoint is <BlogHighLight bold=true>index.html</BlogHighLight> file.</BlogBody>
            <BlogImage border=true src="/assets/images/document/flutter-web-directory.png"/>
            <BlogHighLight bold=true>So the idea is that we will will use Nginx to act like a simple web server and serve our build/web directory to the internet. SIMPLE {"😉"} </BlogHighLight>
            <BlogHeader2>1 - Nginx</BlogHeader2>
            <BlogBody>Copy and paste this file into your directory</BlogBody>
            <CodeBlock code=r#"
api {
    listen 80;

    location / {
        root /app;
        index index.html;
    }
}
            "#/>
            <BlogHeader2>
                2 - Write Dockerfile
            </BlogHeader2>
            <BlogBody>
                If you dont know what Docker is, feel free to visit their website <BlogLink href="https://docs.docker.com/get-started/overview">here</BlogLink>.
                In this scope I will only cover how to write a Dockerfile that support Flutter Web.
            </BlogBody>
            <BlogBody>
                Now, on you project directory create file call <BlogHighLight italic=true bold=true>Dockerfile</BlogHighLight>
            </BlogBody>
            <BlogImage border=true src="/assets/images/document/dockerfile-in-project.png"/>
                <BlogBody>Lets breakdown our Docker instructions before we start</BlogBody>
                <ul class="list-none text-lg list-outside border-pink-900 border-2 rounded-xl">
                    <li>
                        <BlogHeader3 >We need to have two steps: Build and Run, why ? </BlogHeader3>
                        <BlogBody>
                            The build process requires more dependencies than the runtime, resulting in our runtime consuming more storage than necessary.
                            To address this, we can split the process into two phases. After completing the build phase, we can then copy the output into the runtime phase.
                        </BlogBody>
                    </li>
                    <li>
                        <BlogHeader3>[Build] setup the Flutter SDk</BlogHeader3>
                        <BlogBody>We will install Flutter SDk and its dependencies</BlogBody>
                    </li>
                    <li>
                        <BlogHeader3>[Build] build the flutter-web</BlogHeader3>
                        <BlogBody>By using command: <BlogHighLight bold=true border=true>flutter build web --release</BlogHighLight> flutter will build and then the output will be saved into <BlogHighLight bold=true>build/web</BlogHighLight> directory.</BlogBody>
                    </li>
                    <li>
                        <BlogHeader3>[Run] Copy the output of Building phase /build/web into Running phase</BlogHeader3>
                    </li>
                </ul>
            <BlogBody>Copy and paste this into your Dockerfile</BlogBody>
            <CodeBlock language="Dockerfile" code=r#"
FROM ubuntu:16.04 as builder
USER root
# Install Flutter build-time dependencies
RUN apt-get update && \\
    apt-get install -y --no-install-recommends git wget unzip libglu1-mesa lib32stdc++6 ca-certificates curl tar \\
    xz-utils clang cmake ninja-build pkg-config libgtk-3-dev && \\
    rm -rf /var/lib/apt/lists/*
WORKDIR /
# Download and install flutter 3.13.8, feel free to change the version as needed
RUN curl https://storage.googleapis.com/flutter_infra_release/releases/stable/linux/flutter_linux_3.13.8-stable.tar.xz -o flutter-sdk.tar.xz
RUN tar xf flutter-sdk.tar.xz && rm flutter-sdk.tar.xz
ENV PATH="$PATH:/flutter/bin"
RUN flutter config --no-analytics --enable-web && \\
    flutter precache && \\
    flutter doctor && \\
    rm -rf .pub_cache
RUN dart pub global activate protoc_plugin
ENV PATH="$PATH":"/root/.pub-cache/bin/"
# Copy your project into Docker and build the flutter web
WORKDIR /src
COPY . .
RUN flutter build web --release

# Now we switch to the running phase
# In this phase we simply do:
# - Install nginx to use as WebServer
# - Copy the build/web folder from builder to runner
# - Restart nginx everytime we start the container
FROM ubuntu:16.04 as runner
USER root
WORKDIR /app
RUN apt-get update && \\
    apt-get install -y \\
    curl unzip nginx && \\
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /src/build/web .
COPY nginx.conf /etc/nginx/sites-available/default
RUN service nginx stop
ENTRYPOINT ["/bin/bash", "-c", "echo 'Start nginx...'; nginx -g 'daemon off;'"]
"#/>
            <BlogHeader2>
                3 - Test on your local machine
            </BlogHeader2>
            <BlogBody>On your project directory, to build Docker image we run:</BlogBody>
            <CodeBlock language="bash" code=r#"
docker build -t simple_web .
            "#/>
            <BlogBody>
                After run command above, Docker will start to build your Docker image, it could took very long, up to 30 minutes.
            </BlogBody>
            <BlogBody>Your can check your result by using this command</BlogBody>
            <CodeBlock language="bash" code=r#"docker images | grep simple_web"#/>
            <BlogBody>The result should look like this:</BlogBody>
            <CodeBlock language="bash" code=r#"
docker images | grep simple_web
simple_web  latest    873973fd1815   2 minutes ago    222MB
            "#/>
            <BlogBody>Next we need to run Docker container from that image. Enter this command:</BlogBody>
            <CodeBlock language="bash" code=r#"
docker run -p 3000:80 --name simple_web_container simple_web:latest
            "#/>
            <BlogBody>Then go check your <BlogLink href="http://localhost:3000">localhost:3000</BlogLink></BlogBody>
            <BlogBody>If it is online, {"🤗"} you are successfully dockerized your website !!!, we ve completed 70% now !!</BlogBody>
            <BlogHeader>Github Action</BlogHeader>
            <BlogBody>
                We will not delve into the details of what Github Action is and how to use it in this section.
                You can easily follow along with this post without prior knowledge of Github Action. However, I recommend checking out the
                <BlogLink href="https://docs.github.com/en/actions/learn-github-actions/understanding-github-actions">Github Action documentation</BlogLink> for a basic understanding before we proceed.
            </BlogBody>
            <BlogBody newline=true>
                At this point, we&#39;ve successfully dockerized our code, and our cloud server is in place. However, we still need a solution to:
                <ul class="list list-disc">
                    <li><BlogBody>Automatically build every time there are code changes.</BlogBody></li>
                    <li><BlogBody>Deploy to our cloud server.</BlogBody></li>
                </ul>
            </BlogBody>
            <BlogBody>
                To achieve this, we&#39;ll leverage Github Action, which provides us with a virtual computer(free 2000hrs/month).
                This allows us to handle:
                <ul class="list list-disc">
                    <li><BlogBody>Trigger the build process on Github Action.</BlogBody></li>
                    <li><BlogBody>After complete, on Github Action, we connect to our cloud server to execute Docker command.</BlogBody></li>
                    <li><BlogBody>Then use docker command to run a new container from the Docker image that we has been built.</BlogBody></li>
                </ul>
            </BlogBody>
            <BlogBody>Now let get started</BlogBody>
                <BlogHeader2>1 - Allow Github Action to perform ssh</BlogHeader2>
            <BlogBody>
                To enable Github Action to link with our cloud server, we need to authorize it to use SSH for connecting to the server.
            </BlogBody>
            <BlogBody>Now, execute this command on your cloud server to generate the ssh key (skip if you already setup ssh)</BlogBody>
            <CodeBlock language="bash" code=r#"ssh-keygen"#/>
            <BlogBody>Next, execute this command to get your private key</BlogBody>
            <CodeBlock language="bash" code=r#"cat ~/.ssh/id_rsa"#/>
            <BlogBody>Copy the whole output of that command, we need to use it later on.</BlogBody>
            <BlogBody>Open your Github Repository, go to settings, in the left bottom, there is an options call <BlogHighLight bold=true>Secrets and Variables</BlogHighLight></BlogBody>
            <BlogImage border=true src="/assets/images/document/github_secret.png"/>
            <BlogBody>Press the button <BlogHighLight rounded=true background=true>New repository secret</BlogHighLight> which will produce this popup: </BlogBody>
            <BlogImage border=true src="/assets/images/document/github_action_new_secret.png"/>
            <BlogBody>
                Now fill in:
                <ul class="list-none w-full p-2">
                    <li><BlogBody class="pt-2"><BlogHighLight rounded=true border=true>SSH_PRIVATE_KEY / Enter your secret that has been generated above</BlogHighLight> </BlogBody></li>
                    <li><BlogBody class="pt-2"><BlogHighLight rounded=true border=true> SSH_HOST / The public IP address of your cloud server</BlogHighLight></BlogBody></li>
                    <li><BlogBody class="pt-2"><BlogHighLight rounded=true border=true> DOCKER_REGISTRY_USERNAME / Your Docker Hub username</BlogHighLight></BlogBody></li>
                    <li><BlogBody class="pt-2"><BlogHighLight rounded=true border=true> DOCKER_REGISTRY_PASSWORD / Your Docker Hub password</BlogHighLight></BlogBody></li>
                </ul>
            </BlogBody>
            <BlogBody>Hmm... we are now ready to move to the last step {"😃"}.</BlogBody>
            <BlogHeader2>2 - Create the Github workflow</BlogHeader2>
            <BlogBody><BlogLink href="https://docs.github.com/en/actions/using-workflows/about-workflows">Github workflow</BlogLink> allow us to describe our instructions on how to build and deploy our application.</BlogBody>
            <BlogBody>On your project directory create a folder name <BlogHighLight italic=true>.github/workflows/ci.yaml</BlogHighLight></BlogBody>
            <BlogImage border=true src="/assets/images/document/ci-folder.png"/>
            <BlogBody>Copy and paste this into your ci.yaml</BlogBody>
            <CodeBlock language="yaml" code=r#"
on:
  push:
    branches:
      - 'main' # Your master branch
env:
  IMAGE_NAME: ${{ secrets.DOCKER_REGISTRY_USERNAME }}/simple_web # feel free to edit to your project name

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Docker login
        run: echo ${{ secrets.DOCKER_REGISTRY_PASSWORD }} | docker login -u ${{ secrets.DOCKER_REGISTRY_USERNAME }} --password-stdin
      - name: Build Docker image
        run: docker build -t ${{ env.IMAGE_NAME }} .
      - name: Publish Docker image
        run: |
          docker push ${{ env.IMAGE_NAME }}
  deploy:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Deploy
        run: |
          install -m 600 -D /dev/null ~/.ssh/id_rsa
          echo "${{ secrets.SSH_PRIVATE_KEY }}" > ~/.ssh/id_rsa
          ssh-keyscan -H "${{ secrets.SSH_HOST }}" > ~/.ssh/known_hosts
          ssh -tt root@${{ secrets.SSH_HOST }} "docker pull ${{ env.IMAGE_NAME }}:latest && docker rm -f simple_web && docker run -d --rm --name simple_web -p 3000:80 ${{ env.IMAGE_NAME }}:latest"
      - name: Cleanup
        run: rm -rf ~/.ssh
            "#/>
            <BlogBody>
                Now, lets commit and push your code to Github
            </BlogBody>
            <BlogBody>
                Now open your Github Repository, click on tab <BlogHighLight rounded=true background=true>Actions</BlogHighLight>
            </BlogBody>
            <BlogImage border=true src="/assets/images/document/github_action_tab.png"/>
            <BlogBody>
                You will see your workflow is ready right there, and because we are setting the workflow to automatically running every time we push to the main branch,
                Github will now automatically trigger the first build process.
            </BlogBody>
            <BlogImage border=true src="/assets/images/document/github_action_running.png"/>
            <BlogBody>And your CI should be success {"😉"}!!</BlogBody>
            <BlogHeader>Finalize</BlogHeader>
            <BlogBody>
                Thank you for your reading, I will continue to write another blog on how I apply cdn for Flutter Web, looking forward to see you there soon.
            </BlogBody>
        </BlogContainer>
    }
}